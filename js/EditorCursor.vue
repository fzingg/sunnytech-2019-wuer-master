<template>
  <span
    class="editor-cursor"
    v-bind:class="{ blink }"
  ></span>
</template>

<script>
export default {
  props: {
    blink: {
      type: Boolean,
      default: true
    }
  }
}
</script>

<style>
.editor-cursor {
  height: 5px;
  border: 1px solid #aaa;
  margin: -1px;
  z-index: -5;
}

.blink {
  animation: blink 1s step-end 0s infinite;
}

@keyframes blink {
  50% {
    opacity: 0.0;
  }
}
</style>
