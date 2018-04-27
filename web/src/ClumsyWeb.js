import React from 'react'
import Terminal from 'terminal-in-react'

class ClumsyWeb extends React.Component {
  handleInput(input) {
    return this.props.wasm.evaluate(input)
  }

  render() {
    return (
      <Terminal
        color="white"
        prompt="white"
        startState="maximised"
        hideTopBar={true}
        allowTabs={false}
        style={{ fontFamily: 'monospace', fontSize: 'large' }}
        commandPassThrough={input => this.handleInput(input.join(' '))}
      />
    )
  }
}

export default ClumsyWeb
// vim: set ts=2 sw=2 et:
