<template>
  <div class="editors-uis-container">
    <EditorUi
      v-for="editor in editors"
      v-bind:key="editor.title"
      v-bind="editor"
    ></EditorUi>
  </div>
</template>

<script>
import EditorUi from './EditorUi.vue'
export default {
  components: { EditorUi },
  props: ['editors']
}
</script>

<style scoped>
.editors-uis-container {
  padding: 0px 20px;
  margin-top: 30px;
  display: flex;
  justify-content: center;
}
</style>
