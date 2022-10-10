<template>
  <div class="app">

    <div class="buttons">

      <button v-on:click="broadcastContents">
        broadcast contents
      </button>

      <button v-on:click="clearContents">
        clear contents
      </button>

      <button v-on:click="playScenario">
        play scenario
      </button>

      <button v-on:click="showCausalTrees = !showCausalTrees">
        <span v-if="showCausalTrees">hide</span>
        <span v-else>show</span>
        causal trees
      </button>

      <button v-on:click="showSequences = !showSequences">
        <span v-if="showSequences">hide</span>
        <span v-else>show</span>
        sequences
      </button>

    </div>

    <EditorsUis v-bind:editors="editors"/>

    <div class="debug-root">
      <CausalTrees
        v-if="showCausalTrees"
        v-bind:editors="editors"
      />
      <EventsSequences
        v-if="showSequences"
        v-bind:editors="editors"
      />
    </div>

  </div>
</template>

<script>
import EditorsUis from './EditorsUis.vue'
import CausalTrees from './CausalTrees.vue'
import EventsSequences from './EventsSequences.vue'
import ProxyForRustEditor from './ProxyForRustEditor.js'
import player from './player.js'
import editorsManager from './editorsManager.js'

export default {
  components: {
    EditorsUis,
    CausalTrees,
    EventsSequences
  },
  data () {
    return {
      editors: [
        {
          color: '#f8f8f8', titleTextColor: '#444', title: 'Editor 1 - Rudy',
          model: new ProxyForRustEditor()
        },
        {
          color: '#654ff0', titleTextColor: '#eee', title: 'Editor 2 - Walt',
          model: new ProxyForRustEditor()
        },
        {
          color: '#f7df1e', titleTextColor: '#444', title: 'Editor 3 - Jane',
          model: new ProxyForRustEditor()
        },
      ],
      showCausalTrees: false,
      showSequences: false
    }
  },
  methods: {
    broadcastContents () {
      editorsManager.broadcastContents(this.editors)
    },
    clearContents () {
      editorsManager.clearContents(this.editors)
    },
    playScenario () {
      player.playScenario(this.editors)
    }
  },
  async created () {
    let wasmForRustModule = await import("../crate/pkg")
    for ( const editor of this.editors ) {
      editor.model.instantiateRustEditor(wasmForRustModule)
      editor.model.clear()
    }
  }
}
</script>

<style>
.buttons {
  margin: 20px;
  display: flex;
  justify-content: center;
}

.buttons button {
  margin: 0px 10px;
  color: #555;
  background: #eee;
  padding: 6px 15px;
  border-radius: 5px;
  display: inline-block;
  border: none;
  cursor: pointer;
}

button::-moz-focus-inner {
  border: 0;
}

button:hover {
  color: white;
  background-color: #aaa;
}

button:active {
  transform: translateY(2px);
}


.debug-root {
  margin: 40px;
}
</style>

<style scoped>
.app {
  position: absolute;
  margin: auto;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  font-family: SegoeUI, Tahoma, Geneva, Verdana, sans-serif;
  font-size: 17px;
  color: #444;
}
</style>
