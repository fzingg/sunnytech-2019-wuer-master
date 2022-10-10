export default {
  broadcastContents (editors) {
    for ( const editor1 of editors ) {
      for ( const editor2 of editors ) {
        if (editor1 !== editor2) {
          editor1.model.send(editor2.model)
        }
      }
    }
  },
  clearContents (editors) {
    for ( const editor of editors ) {
      editor.model.clear()
    }
  }
}
  