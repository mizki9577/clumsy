import React from 'react'
import { Window, TitleBar } from 'react-desktop/macOs'
import styled from 'styled-components'

const ClumsyWeb = () => (
  <Window chrome background="black" width="100%" height="100%" padding="none">
    <TitleBar title="Clumsy" controls inset />
    <Terminal prompt="> " />
  </Window>
)

const Wrapper = styled.div`
  width: 100%;
  height: 4.5in;
  overflow-y: scroll;
  font-family: monospace;
  font-size: 12pt;
  color: white;
  background-color: black;
  white-space: pre-wrap;
  word-break: break-all;
`

const Input = styled.input.attrs({ type: 'text' })`
  font-family: monospace;
  font-size: 12pt;
  color: white;
  background-color: black;
  border: none;
  outline: none;
`

class Terminal extends React.Component {
  constructor(props = { prompt: '> ' }) {
    super(props)

    this.state = {
      history: [''],
      historyIndex: 0,
      stdout: ['Please wait...'],
      ready: false,
    }
  }

  componentDidMount() {
    import('./clumsy_web.js')
      .then(wasm => {
        this.wasm = wasm
        this.setState({ ready: true, stdout: [''] })
      })
      .catch(x => {
        this.setState({
          stdout: [x.toString()],
        })
      })
  }

  componentDidUpdate() {
    this.input.scrollIntoView()
  }

  handleKeyDown(ev) {
    const input = ev.target.value
    let { history, historyIndex, stdout } = this.state
    const { prompt } = this.props

    switch (ev.key) {
      case 'Enter':
        history = ['', ...history]
        historyIndex = 0
        stdout = [...stdout, prompt + input, this.wasm.evaluate(input)]
        break

      case 'ArrowUp':
        historyIndex = Math.min(history.length - 1, historyIndex + 1)
        history = [history[historyIndex], ...history.slice(1)]
        break

      case 'ArrowDown':
        historyIndex = Math.max(0, historyIndex - 1)
        history = [history[nextHistoryIndex], ...history.slice(1)]
        break

      default:
        historyIndex = 0
        break
    }

    this.setState({ history, historyIndex, stdout })
  }

  handleChange(ev) {
    this.setState({
      history: [ev.target.value, ...this.state.history.slice(1)],
    })
  }

  render() {
    return (
      <Wrapper
        onClick={() => {
          this.input.focus()
        }}
      >
        {this.state.stdout.map((line, i) => <div key={i}>{line}</div>)}
        {this.state.ready ? this.props.prompt : null}
        <Input
          innerRef={x => {
            this.input = x
          }}
          value={this.state.history[0]}
          onKeyDown={ev => this.handleKeyDown(ev)}
          onChange={ev => this.handleChange(ev)}
        />
      </Wrapper>
    )
  }
}

export default ClumsyWeb
// vim: set ts=2 sw=2 et:
