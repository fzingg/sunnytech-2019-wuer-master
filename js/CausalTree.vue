<template>
  <div>
    <CausalTreeNode
      v-bind:node=" sequence | causalTreeRoot "
      v-bind:cursor="cursor"
    />
  </div>
</template>

<script>
import CausalTreeNode from './CausalTreeNode.vue'

export default {
  components: {
    CausalTreeNode
  },
  props: ['sequence', 'cursor'],
  filters: {
    causalTreeRoot (flatSequence) {
      const moment2key = (moment) => 
        `${moment.editor_id}-${moment.timestamp}`
      const nodes = {}
      let treeRoot
      if (flatSequence) {
        flatSequence.forEach(
          event => {
            const keyString = moment2key(event.key.moment)
            const node = {
              ...event,
              keyString,
              children: []
            }
            nodes[keyString] = node
            if (event.key.moment.editor_id === -1) {
              // Origin event is the only one which editor id is -1.
              treeRoot = node
            } else {
              nodes[moment2key(event.cause.moment)].children.push(node)
            }
          } 
        )
      }
      return treeRoot
    }
  }
}
</script>