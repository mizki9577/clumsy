import React from 'react'
import ReactDOM from 'react-dom'
import clumsy from './clumsy.rs'

const PROMPT = '>>>\u00A0'

const evaluate = source => {
  const size = source.length + 1  // NOTE: non-ASCII character is not considered
  const source_ptr = clumsy.alloc(source.length)
  const source_array = new Int8Array(clumsy.memory.buffer, source_ptr, size)

  source_array.set(Array.from(source).map(c => c.charCodeAt(0)))
  source_array[size - 1] = 0  // terminating by null

  const result_ptr = clumsy.eval(source_ptr)
  const result_array = new Int8Array(clumsy.memory.buffer, result_ptr)

  let i = 1
  while (result_array[i] != 0) {
    ++i
  }

  const result = String.fromCodePoint(...result_array.slice(1, i + 1))
  return result
}

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
      <div className="clumsy-web">
        <div>
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

ReactDOM.render(
  <ClumsyWeb />,
  document.querySelector('main'),
)

// vim: set ts=2 sw=2 et:
