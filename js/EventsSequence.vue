<template>
  <div class="sequence">

    <span style="width: 120px; color: #999;">{{ title }}</span>

    <span
      v-for="(event, index) in sequence"
      v-bind:key="`sequence-item-${index}`"
    >
      <EditorEvent
        v-bind:event="event"
        display-cause
        v-bind:highlight="cursorIsAtCurrentEvent(event)"
        class="event"
      />
    </span>

  </div>
</template>

<script>
import EditorEvent from './EditorEvent.vue'

export default {
  components: { EditorEvent },
  props: {
    title: String,
    sequence: Array,
    cursor: Object
  },
  methods: {
    cursorIsAtCurrentEvent (event) {
      return (
        event.key.moment.editor_id === this.cursor.editor_id
        &&
        event.key.moment.timestamp === this.cursor.timestamp
      )
    }
  }
}
</script>

<style scoped>
.sequence {
  display: flex;
  flex-wrap: wrap;
  margin: 10px;
}

.event {
  margin: 2px;
}
</style>