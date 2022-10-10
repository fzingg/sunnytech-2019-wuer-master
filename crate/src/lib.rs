#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

cfg_if::cfg_if! {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function to get better error messages if we ever panic.
  if #[cfg(feature = "console_error_panic_hook")] {
    use console_error_panic_hook::set_once as set_panic_hook;
  } else {
    #[inline]
    fn set_panic_hook() {}
  }
}

#[wasm_bindgen(start)]
pub fn init() {
  set_panic_hook()
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn console_log(s: &str);
} 

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp::PartialEq;
use std::cmp::Ordering;
use std::cmp::PartialOrd;

const BACKSPACE_CHARACTER: char = '\x08';
const ORIGIN_CHARACTER: char = '\t';

fn editor_id_to_letter(editor_id: i32) -> char {
  match editor_id {
    0 => 'r',
    1 => 'w',
    2 => 'j',
    _ => '.'
  }
}

//-----------------------------------------------

type Timestamp = u32;

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, Hash, Serialize)]
pub struct Moment {
  editor_id: i32,
  timestamp: Timestamp,
}

impl PartialEq for Moment {
  fn eq(&self, other: &Self) -> bool {
    self.editor_id == other.editor_id
    &&
    self.timestamp == other.timestamp
  }
}

impl PartialOrd for Moment {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Moment {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.timestamp == other.timestamp {
      self.editor_id.cmp(&other.editor_id)
    } else {
      // Reverse time order
      other.timestamp.cmp(&self.timestamp)
    }
  }
}

//-----------------------------------------------

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, Hash, Serialize)]
pub struct EventKey {
  moment: Moment,
  priority: u32,
}

impl PartialEq for EventKey {
  fn eq(&self, other: &Self) -> bool {
    self.moment == other.moment
  }
}

impl PartialOrd for EventKey {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for EventKey {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.priority == other.priority {
      self.moment.cmp(&other.moment)
    } else {
      // Reverse priority order
      other.priority.cmp(&self.priority)
    }
  }
}

const VERY_LARGE_PRIORITY : u32 = 1000;

const ORIGIN_KEY: EventKey = EventKey {
  moment: Moment {
    editor_id: -1,
    timestamp: 0
  },
  priority: VERY_LARGE_PRIORITY
};

//-----------------------------------------------

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, Hash, Serialize)]
struct Event {
  key: EventKey,
  cause: EventKey,
  character: char
}

impl std::cmp::PartialEq for Event {
  fn eq(&self, other: &Self) -> bool {
    self.key == other.key
  }
}

impl PartialOrd for Event {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Event {
  fn cmp(&self, other: &Self) -> Ordering {
    self.key.cmp(&other.key)
  }
}

const ORIGIN_EVENT : Event = Event {
  key: ORIGIN_KEY, cause: ORIGIN_KEY, character: ORIGIN_CHARACTER
};

//-----------------------------------------------

type CausalSet = BTreeSet<Event>;

#[wasm_bindgen]
pub struct Content {
  // A plain sequence of events ordered by their keys.
  sequence: BTreeMap<EventKey, Event>,
 
  // Keep track of cause -> events list relationship (the other way of the
  // relationship is tracked in each individual event, with the "cause" field)
  causal_sets: BTreeMap<EventKey, CausalSet>,
}

#[wasm_bindgen]
impl Content {

  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let mut sequence = BTreeMap::new();
    sequence.insert(ORIGIN_EVENT.key, ORIGIN_EVENT);
    Self {
      sequence,
      causal_sets: BTreeMap::new(),
    }
  }

  // For a given new event, add it to the causal_set of its cause.
  fn update_causal_set(&mut self, event: &Event) {
    let causal_set =
      self.causal_sets
        .entry(event.cause)
        .or_insert(CausalSet::new());
    causal_set.insert(*event);
  }

  fn add_event(&mut self, event: &Event) {
    self.sequence.insert(event.key, *event);
    self.update_causal_set(event);
  }

  // Depth-first "flatten" of the causal tree starting at a given cause.
  fn flatten(&self, cause: &EventKey) -> Vec<&Event> {
    let mut flat = match self.sequence.get(cause) {
      Some(event) => vec![event],
      None => Vec::new(),
    };
    match self.causal_sets.get(cause) {
      None => flat,
      Some(children) => {
        for event in children.iter() {
          if event.key != ORIGIN_KEY {
            // Avoid loops. This happens only for origin which cause is itself.
            flat.append(&mut self.flatten(&event.key));
          }
        }
        flat
      }
    }
  }

  // Depth-first "flatten" of the causal tree starting at its root.
  fn flatten_all(&self) -> Vec<&Event> {
    let root = self.sequence.iter().next();
    match root {
      None => Vec::new(),
      Some((first_event_key, _)) => {
        self.flatten(first_event_key)
      }
    } 
  }

  // Only keep the events that actually will produce some output. This means go
  // sequentially through the flatten list, pushing events and removing previous
  // push if event is a BACKSPACE.
  fn get_final_events_stream(&self) -> Vec<Event> {
    let mut events : Vec<Event> = Vec::new();
    for event in self.flatten_all() {
      if event.key != ORIGIN_KEY {
        if event.character == BACKSPACE_CHARACTER {
          match events.last() {
            Some(previous_event) => {
              if previous_event.key == event.cause {
                events.pop();
              }
            }
            None => {}
          }
        } else {
          events.push(*event);
        }
      }
    }
    events
  }

  // Actually produce the characters of the current state of document.
  #[allow(dead_code)]
  pub fn get_characters(&self) -> String {
    let events = self.get_final_events_stream();
    events
      .iter()
      .map(|event| {
        event.character
      })
      .collect::<String>()
  }

  pub fn send(&self, recipient: &mut Content) {
    for (key, event) in self.sequence.iter() {
      recipient.add_event(&Event {
        key: *key,
        cause: event.cause,
        character: event.character
      });
    }
  }

  pub fn get_flat_sequence(&self) -> JsValue {
    JsValue::from_serde(&self.flatten_all()).unwrap()
  }
}

//-----------------------------------------------

#[wasm_bindgen]
struct Cursor {
  position: usize,
  event_key: EventKey
}

const ORIGIN_CURSOR : Cursor = Cursor {
  position: 0,
  event_key: ORIGIN_KEY
};

#[wasm_bindgen]
pub struct Editor {
  editor_id: i32,
  current_timestamp: u32,
  cursor: Cursor,
  content: Content,
  // The final events stream is often needed (for cursor moves, for instance)
  // and its computation can be time consuming so we cache it. The drawback: we
  // have to be sure to update it every time content is changed.
  final_events_stream: Vec<Event>,
  model_to_update_on_change: JsValue
}

#[wasm_bindgen]
impl Editor {

  #[wasm_bindgen(constructor)]
  #[allow(dead_code)]
  pub fn new(editor_id: i32, model_to_update_on_change: JsValue) -> Self {
    let content = Content::new();
    let final_events_stream = content.get_final_events_stream();
    Self {
      editor_id,
      current_timestamp: 1,
      cursor: ORIGIN_CURSOR,
      content,
      final_events_stream,
      model_to_update_on_change
    }
  }

  fn next_timestamp(&mut self) -> Timestamp {
    let timestamp = self.current_timestamp;
    self.current_timestamp += 1;
    timestamp
  }

  fn next_moment(&mut self) -> Moment {
    Moment {
      editor_id: self.editor_id,
      timestamp: self.next_timestamp()
    }
  }

  fn next_event_key(&mut self, character: char) -> EventKey {
    EventKey {
      moment: self.next_moment(),
      priority: match character {
        BACKSPACE_CHARACTER => 2,
        _ => 1
      }
    }
  }

  fn add_to_content(&mut self, character: char) {
    let key = self.next_event_key(character);
    self.content.add_event(&Event {
      key,
      cause: self.cursor.event_key,
      character
    });
    self.final_events_stream = self.content.get_final_events_stream();
    self.notify_change()
  }

  #[allow(dead_code)]
  pub fn insert_character(&mut self, character: char) {
    self.add_to_content(character);
    if character == BACKSPACE_CHARACTER {
      self.move_cursor_left();
    } else {
      self.move_cursor_right();
    }
  }

  #[allow(dead_code)]
  pub fn remove_previous_character(&mut self) {
    self.insert_character(BACKSPACE_CHARACTER);
  }

  pub fn update_cursor_event_key(&mut self) {
    self.cursor.event_key =
      match self.cursor.position {
        0 => ORIGIN_KEY,
        _ => match self.final_events_stream.get(self.cursor.position - 1) {
          Some(event) => event.key,
          None => self.cursor.event_key
        }
      };
  }

  #[allow(dead_code)]
  pub fn move_cursor_right(&mut self) {
    self.cursor.position =
      std::cmp::min(
        self.cursor.position + 1,
        self.final_events_stream.len()
      );
    self.update_cursor_event_key();
    self.notify_change()
  }

  #[allow(dead_code)]
  pub fn move_cursor_left(&mut self) {
    if self.cursor.position >= 1 {
      self.cursor.position = self.cursor.position - 1;
      self.update_cursor_event_key();
      self.notify_change()
    }
  }

  #[allow(dead_code)]
  pub fn clear(&mut self) {
    self.content = Content::new();
    self.final_events_stream = self.content.get_final_events_stream();
    self.cursor = ORIGIN_CURSOR;
    self.notify_change()
  }

  fn find_and_set_new_cursor_position(&mut self) {
    let mut position = 1;
    for event in self.final_events_stream.iter() {
      if event.key == self.cursor.event_key {
        self.cursor.position = position;
      }
      position += 1;
    }
  }

  #[allow(dead_code)]
  pub fn receive_events_from(&mut self, sender: &mut Editor) {
    sender.content.send(&mut self.content);
    self.final_events_stream = self.content.get_final_events_stream();
    self.find_and_set_new_cursor_position();
    self.current_timestamp =
      1 + std::cmp::max(
        sender.current_timestamp,
        self.current_timestamp
      );
    self.notify_change()
  }

  #[allow(dead_code)]
  pub fn send_events_to(&mut self, recipient: &mut Editor) {
    recipient.receive_events_from(self);
  } 

  #[allow(dead_code)]
  fn add_editor_infos_to_dom(&self) {
    let window = web_sys::window().expect("global `window` not found");
    let document = window.document().expect("window.document not found");

    let dom_element_id = format!("editor-infos-{}", &self.editor_id);
    match document.get_element_by_id(&dom_element_id) {
      None => {
        console_log(&format!(
          "Warning: can't find DOM element with id = {} to fill with editor's infos, skip DOM update until element exits",
          &dom_element_id
        ));
      }
      Some(element) => {
        element.set_inner_html(&format!(
          "
            <div> timestamp {} - length {} </div>
            <div>
              <span style='padding-right: 10px;'>cursor</span>
              <span style='padding-right: 10px;'>{}</span>
              {}{}
            </div>
          ",
          &self.current_timestamp,
          &self.final_events_stream.len(),
          &self.cursor.position,
          editor_id_to_letter(self.cursor.event_key.moment.editor_id),
          &self.cursor.event_key.moment.timestamp
        ));
      }
    }
  }

  fn notify_change(&self) {
    let _ = js_sys::Reflect::set(
      &self.model_to_update_on_change,
      &"content".into(),
      &JsValue::from(self.content.get_characters())
    );
    let _ = js_sys::Reflect::set(
      &self.model_to_update_on_change,
      &"cursorPosition".into(),
      &JsValue::from(self.cursor.position as u32)
    );
    let _ = js_sys::Reflect::set(
      &self.model_to_update_on_change,
      &"cursorMoment".into(),
      &JsValue::from_serde(&self.cursor.event_key.moment).unwrap()
    );
    let _ = js_sys::Reflect::set(
      &self.model_to_update_on_change,
      &"flatSequence".into(),
      &self.content.get_flat_sequence()
    );
    let _ = js_sys::Reflect::set(
      &self.model_to_update_on_change,
      &"finalSequence".into(),
      &JsValue::from_serde(&self.final_events_stream).unwrap()
    );
    self.add_editor_infos_to_dom();
  }
}
