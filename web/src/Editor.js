import React from 'react'
import { Paper } from '@material-ui/core'
import AceEditor from 'react-ace'
import 'brace'
import 'brace/keybinding/vim'
import 'brace/keybinding/emacs'

const Editor = ({ value, keyboardHandler, onChange, onEvaluation }) => (
  <Paper>
    <AceEditor
      width="100%"
      value={value}
      keyboardHandler={keyboardHandler}
      onChange={onChange}
      commands={[{ bindKey: 'Ctrl-Enter', exec: () => onEvaluation() }]}
    />
  </Paper>
)

export default Editor
// vim: set ts=2 sw=2 et:
