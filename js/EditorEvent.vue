<template>
  <div style="display: inline-block;">

    <div v-bind:class="`event-moments`">
      <MomentTag
        class="top-moment-tag"
        v-bind:moment="top.moment"
      />
      <MomentTag
        v-if="middle"
        class="middle-moment-tag"
        v-bind:moment="middle.moment"
      />
    </div>

    <div
      v-bind:class="{
        'event-character': true,
        'highlight': highlight
      }">
      {{ event.character | toDisplayable }}
    </div>

  </div>
</template>

<script>
import MomentTag from './MomentTag.vue'

export default {
  components: { MomentTag },
  props: {
    event: Object,
    displayCause: Boolean,
    highlight: Boolean
  },
  computed: {
    top () {
      return this.displayCause ? this.event.cause : this.event.key
    },
    middle () {
      return this.displayCause ? this.event.key : undefined
    }
  },
  filters: {
    toDisplayable (character) {
      switch (character) {
        case ' ' : return '·' ; break ;
        case '\n': return '⤶' ; break ;
        case '\b': return '⨂' ; break ;
        case '\t': return '⚑' ; break ;
        default: return character
      } 
    }
  }
}
</script>

<style scoped>
.event-moments {
  font-size: smaller;
  font-weight: bold;
  text-align: center;
}

.top-moment-tag,
.middle-moment-tag {
  padding: 0px 7px;
} 

.top-moment-tag {
  border-top-left-radius: 6px;
  border-top-right-radius: 6px;
}

.event-character {
  font-weight: bold;
  font-size: smaller;
  color: #444;
  text-align: center;

  padding: 0px 7px;
  padding-bottom: 2px;
  background: #ddd;
  border-bottom-left-radius: 6px;
  border-bottom-right-radius: 6px;
}

.highlight {
  box-shadow: 0px 4px 0px #f66;
}
</style>
