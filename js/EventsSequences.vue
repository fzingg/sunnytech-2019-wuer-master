<template>
  <div>

    <div
      v-for="(editor, index) in editors"
      v-bind:key="`debug-editor-${index}`"
    >

      Editor {{ index }} 

      <EventsSequence
        v-if="editor.model && editor.model.flatSequence"
        title="all events"
        v-bind:sequence="editor.model.flatSequence"
        v-bind:cursor="editor.model.cursorMoment"
      />

      <EventsSequence
        v-if="editor.model && editor.model.flatSequence"
        title="final events"
        v-bind:sequence="editor.model.finalSequence"
        v-bind:cursor="editor.model.cursorMoment"
      />

    </div>

  </div>
</template>

<script>
import EventsSequence from './EventsSequence.vue'

export default {
  components: { EventsSequence },
  props: ['editors']
}
</script>