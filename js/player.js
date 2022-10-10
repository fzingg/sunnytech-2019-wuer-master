import editorsManager from './editorsManager.js'

const replaySpeed = 4
const replayStep = 1000/replaySpeed

const delay = duration => new Promise(
  resolve => setTimeout(resolve, duration)
)

const playKeyboardSequence = async function (editor, sequence) {
  for ( const character of sequence.split('') ) {
    await delay(replayStep)
    switch(character) {
      case '→': editor.model.moveCursorRight() ; break
      case '←': editor.model.moveCursorLeft() ; break
      default: editor.model.insertCharacter(character)
    }
  }
}

export default {
  async playScenario (editors) {
    await playKeyboardSequence(editors[0], 'Rust')
    await delay(replayStep) ; editorsManager.broadcastContents(editors)
  
    await playKeyboardSequence(editors[1], '→→→→ wasm')
    await playKeyboardSequence(editors[2], '→→→→ Js')
    await delay(replayStep) ; editorsManager.broadcastContents(editors)
  
    await playKeyboardSequence(editors[0], '→→\bW')
    await playKeyboardSequence(editors[1], '→→→\bS')
    await playKeyboardSequence(editors[2], '←←\b\n')  
    await delay(replayStep) ; editorsManager.broadcastContents(editors)
  }  
}
