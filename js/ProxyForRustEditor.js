let editorId = 0

export default class {
  constructor () {
    this.content = ''
    this.cursorPosition = 0
    this.cursorMoment = { editor_id: 1, timestamp: 1 }
    this.flatSequence = undefined
    this.finalSequence = undefined
  }

  instantiateRustEditor (wasmForRustModule) {
    const modelToUpdateOnChange = this
    this.editorId = editorId
    this.rustEditor = new wasmForRustModule.Editor(editorId, modelToUpdateOnChange)
    editorId++
  }

  insertCharacter (character) {
    this.rustEditor.insert_character(character)
  }
  moveCursorRight () {
    this.rustEditor.move_cursor_right()
  }
  moveCursorLeft () {
    this.rustEditor.move_cursor_left()
  }
  clear () {
    this.rustEditor.clear()
  }
  send (recipient) {
    this.rustEditor.send_events_to(recipient.rustEditor)
  }
}
