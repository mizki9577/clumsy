import React from 'react'
import styled from 'styled-components'

const PROMPT = '>>>\u00A0'

const Root = styled.div`
  background: black;
  border-radius: 0.125in;
  border: 0.125in solid black;
  color: white;
  font-family: monospace;
  height: 4.5in;
  overflow-y: scroll;
`

const History = styled.div`
  white-space: pre;
`

const FlexContainer = styled.div`
  display: flex;
`

const Prompt = styled.input`
  background: black;
  border: none;
  color: white;
  font-family: inherit;
  font-size: 100%;
  outline: none;
  width: 100%;
`

class ClumsyWeb extends React.Component {
  constructor(props) {
    super(props)

    this.prompt = React.createRef()
    this.state = {
      input: '',
      outputs: [],
    }
  }

  componentDidMount() {
    this.focusToPrompt()
  }

  focusToPrompt() {
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
        outputs: [
          ...this.state.outputs,
          PROMPT + input,
          this.props.wasm.evaluate(input),
        ],
      })
    }
  }

  render() {
    return (
      <Root onClick={() => this.focusToPrompt()}>
        <History>
          {this.state.outputs.map((str, i) => <div key={i}>{str}</div>)}
        </History>
        <FlexContainer>
          <span>{PROMPT}</span>
          <Prompt
            innerRef={this.prompt}
            type="text"
            value={this.state.input}
            onChange={ev => this.handleChange(ev)}
            onKeyDown={ev => this.handleKeyDown(ev)}
          />
        </FlexContainer>
      </Root>
    )
  }
}

export default ClumsyWeb
// vim: set ts=2 sw=2 et:
