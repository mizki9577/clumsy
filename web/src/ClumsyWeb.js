import React from 'react'
import { evaluate } from './lib.js'

const PROMPT = '>>>\u00A0'

class ClumsyWeb extends React.Component {
  constructor() {
    super()

    this.prompt = React.createRef()
    this.state = {
      input: '',
      outputs: [],
    }
  }

  componentDidMount() {
    this.prompt.current.focus()
  }

  handleClick() {
    this.prompt.current.focus()
  }

  componentDidUpdate() {
    this.prompt.current.scrollIntoView()
  }

  handleChange(ev) {
    this.setState({
      input: ev.target.value,
    })
  }

  handleKeyDown(ev) {
    if (ev.key === 'Enter') {
      const input = ev.target.value
      this.setState({
        input: '',
        outputs: [...this.state.outputs, PROMPT + input, evaluate(input)]
      })
    }
  }

  render() {
    return (
      <div className="clumsy-web" onClick={ () => this.handleClick() }>
        <div style={{ whiteSpace: 'pre' }}>
          { this.state.outputs.map((str, i) => <div key={ i }>{ str }</div>) }
        </div>
        <div style={{ display: 'flex' }}>
          <span>{ PROMPT }</span>
          <input
            className="prompt"
            type="text"
            value={ this.state.input }
            ref={ this.prompt }
            onChange={ ev => this.handleChange(ev) }
            onKeyDown={ ev => this.handleKeyDown(ev) }
          />
        </div>
      </div>
    )
  }
}

export default ClumsyWeb
// vim: set ts=2 sw=2 et:
