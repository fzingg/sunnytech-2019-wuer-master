<template>
  <div
    class="editor-ui-root"
    v-on:keydown="keydown"
    tabindex=0
    v-on:focus="focused = true"
    v-on:blur="focused = false"
  >

    <div
      class="title"
      v-bind:style="{
        backgroundColor: color,
        color: titleTextColor,
      }"
    > {{ title }} </div>

    <div
      v-if="model"
      class="content"
    >
      <span
        v-for="(character, index) in characters"
        v-bind:key="`${title}-content-char-${index}`"
        v-bind:class="{'punctuation-character': isPunctuation(character)}"
      >
        <EditorCursor
          v-bind:blink="focused"
          v-if="model.cursorPosition === index"
        ></EditorCursor>{{ character | toDisplayable }}</span>
      <EditorCursor
        v-bind:blink="focused"
        v-if="cursorIsAtEndOfContent"
      />
    </div>

    <div class="infos">
      <div v-bind:id="`editor-infos-${model.editorId}`"></div>
    </div>

  </div>
</template>

<script>
import EditorCursor from './EditorCursor.vue'

export default {
  components: { EditorCursor },
  props: {
    model: { type: Object, required: true },
    title: { type: String, required: true },
    color: { type: String, default: '#f8f8f8' },
    titleTextColor: { type: String, default: '#444' },
  },
  data () {
    return {
      focused: false
    }
  },
  computed: {
    characters () {
      return this.model.content.split('')
    },
    cursorIsAtEndOfContent () {
      return this.model.cursorPosition === this.model.content.length
    }
  },
  filters: {
    toDisplayable (character) {
      switch (character) {
        case ' ' : return ' · '  ; break ;
        case '\n': return ' ⤶\n' ; break ;
        default: return character
      }
    }
  },
  methods: {
    keydown (event) {
      if (event.key === 'ArrowRight') {
        this.model.moveCursorRight()
      } else if (event.key === 'ArrowLeft') {
        this.model.moveCursorLeft()
      } else if (event.key === 'Enter') {
        this.model.insertCharacter('\n')
      } else if (event.key === 'Backspace') {
        this.model.insertCharacter('\b')
      } else {
        if (isAPrintableKeyCode(event.keyCode)) {
          this.model.insertCharacter(event.key)
        }
      }
      event.preventDefault()
      event.stopPropagation()
    },
    isPunctuation (character) {
      return ['\n', ' '].includes(character)
    }
  },
}

const isAPrintableKeyCode = (keyCode) => {
  // TODO not sure the following is robust enough.
  const valid = 
    (keyCode > 47 && keyCode < 58)   || // number keys
    (keyCode == 32) ||                  // spacebar
    (keyCode > 59 && keyCode < 64)   || // Some other keys like < > =
    (keyCode > 64 && keyCode < 91)   || // letter keys
    (keyCode > 95 && keyCode < 112)  || // numpad keys
    (keyCode > 159 && keyCode < 171) || // Some other keys (like !, =)
    (keyCode > 185 && keyCode < 193) || // ;=,-./` (in order)
    (keyCode > 218 && keyCode < 223)    // [\]' (in order)

  return valid;
}
</script>

<style scoped>
.editor-ui-root {
  margin: 0 20px;
  width: 170px;
  min-height: 130px;
  border: #ddd 1px solid;
  border-radius: 3px;
  box-shadow: 3px 3px 10px #ddd;
  outline: 0;
  position: relative;
}

.title {
  padding: 4px 15px;
  font-weight: bold;
  border-bottom: #eeeeee 1px solid;
}

.content {
  padding: 15px;
  height: 100px;;
  white-space: pre-line;
}

.punctuation-character {
  color: #aaa;
}

.infos {
  width: 100%;
  height: 42px;
  border-top: 1px solid #eee;
  font-size: smaller;
  color: #888;
  position: absolute;
  bottom: 0px;
  padding-left: 5px;
  padding-top: 2px;
}
</style>