<template>
  <div v-bind:class="`event-editor-${moment.editor_id}`">
    {{ moment.editor_id | toEditorLetter }}
    {{ moment.timestamp }}
  </div>
</template>

<script>
export default {
  props: {
    moment: Object
  },
  filters: {
    toEditorLetter (editorId) {
      switch (editorId) {
        case 0: return 'r' ; break ;
        case 1: return 'w' ; break ;
        case 2: return 'j' ; break ;
        default: return ''
      } 
    }
  }
}
</script>

<style scoped>
.event-editor--1 {
  background: #666;
  color: #eee;
}

.event-editor-0 {
  background: #f8f8f8;
  color: #444;
  border-left: 1px solid #dddddd;
  border-right: 1px solid #dddddd;
}

/* For element with light background which is tagged as a top element (from the
outside of this element) we have to also add a border on top */
.event-editor-0.top-moment-tag {
  border-top: 1px solid #dddddd;
}

.event-editor-1 {
  background: #654ff0;
  color: #eee;
}

.event-editor-2 {
  background: #f7df1e;
  color: #444;
}
</style>