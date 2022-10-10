<template>
  <div>

    <!-- The current node -->
    <div
      class="node-base"
      v-bind:class="{'with-bar': node.children.length > 1}"
    >
      <EditorEvent
        v-bind:event="node"
        v-bind:highlight="cursorIsAtCurrentEvent"
      />
    </div>

    <!-- Its children (if any) -->
    <div style="display: flex; justify-content: space-between;">
      <div
        v-for="node in node.children"
        v-bind:key="node.keyString"
      >
        <CausalTreeNode
          v-bind:node="node"
          v-bind:cursor="cursor"
        />
      </div>
    </div>

  </div>
</template>

<script>
import EditorEvent from './EditorEvent.vue'

export default {
  name: 'CausalTreeNode',
  components: { EditorEvent },
  props: ['node', 'cursor'],
  computed: {
    cursorIsAtCurrentEvent () {
      return (
        this.node.key.moment.editor_id === this.cursor.editor_id
        &&
        this.node.key.moment.timestamp === this.cursor.timestamp
      )
    }
  }
}
</script>

<style scoped>
.node-base {
  margin: 0px 6px;
  padding: 3px 0px;
  text-align: center;
}
.node-base.with-bar {
  border-bottom: 2px solid #ccc;
}
.node-base:not(.with-bar) {
  border-bottom: 2px solid rgba(255, 255, 255, .0);
}
</style>
