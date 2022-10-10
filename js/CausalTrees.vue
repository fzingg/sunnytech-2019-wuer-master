<template>
  <div class="causal-trees-container">

    <div
      v-for="(editor, index) in editors"
      v-bind:key="`causal-tree-${index}`"
      class="causal-tree"
    >

      <center> Editor {{ index }} </center>
      <CausalTree
        v-if="editor.model && editor.model.flatSequence"
        v-bind:sequence="editor.model.flatSequence"
        v-bind:cursor="editor.model.cursorMoment"
        style="margin-top: 10px;"
      />

    </div>

  </div>
</template>

<script>
import CausalTree from './CausalTree.vue'

export default {
  components: { CausalTree },
  props: ['editors']
}
</script>

<style scoped>
.causal-trees-container {
  padding: 0px 20px;
  margin-top: 30px;
  display: flex;
  justify-content: center;
}

.causal-tree {
  margin: 20px;
}
</style>
