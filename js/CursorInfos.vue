<template>
  <div class="cursor-debug-infos">
    <span style="padding-right: 10px;">cursor</span>
    <span style="padding-right: 10px;">{{ position }}</span>
    {{ moment.editor_id | editorLetter }}{{ moment.timestamp }}
  </div>
</template>

<script>
export default {
  props: ['position', 'moment'],
  filters: {
    editorLetter (editorId) {
      if (editorId === 0) {
        return 'r'
      } else if (editorId === 1) {
        return 'w'
      } else if (editorId === 2)  {
        return 'j'
      } else {
        return ''
      }
    }    
  }
}
</script>


<style>
.cursor-debug-infos {
  position: absolute;
  bottom: 0px;
  font-size: smaller;
  color: #888;
  padding-left: 10px;
  padding-bottom: 4px;
}
</style>
