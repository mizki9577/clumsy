import React from 'react'
import { CssBaseline, Grid, Paper } from '@material-ui/core'
import { withStyles } from '@material-ui/core/styles'
import TitleBar from './TitleBar.js'
import MenuBar from './MenuBar.js'
import Editor from './Editor.js'
import Result from './Result.js'

const styles = () => ({
  row: {
    width: '100%',
    margin: 0,
  },
})

class App extends React.Component {
  constructor(props) {
    super(props)

    this.state = {
      source: localStorage.getItem('source') || '',
      keyboardHandler: localStorage.getItem('keyboardHandler'),
      ready: false,
      result: null,
    }
  }

  componentDidMount() {
    import('./clumsy_web.js').then(wasm => {
      this.wasm = wasm
      this.setState({ ready: true })
    })
  }

  handleConfigChange(state) {
    this.setState({ keyboardHandler: state.keyboardHandler })
    localStorage.setItem('keyboardHandler', state.keyboardHandler)
  }

  handleEditorUpdate(source) {
    this.setState({ source })
    localStorage.setItem('source', source)
  }

  handleEvaluation() {
    if (this.state.ready) {
      this.setState({ result: this.wasm.evaluate(this.state.source) })
    }
  }

  render() {
    const { classes } = this.props
    return (
      <React.Fragment>
        <CssBaseline />

        <TitleBar />

        <MenuBar
          className={classes.row}
          keyboardHandler={this.state.keyboardHandler}
          onChange={state => this.handleConfigChange(state)}
          onEvaluation={() => this.handleEvaluation()}
        />

        <Grid container className={classes.row} spacing={16}>
          <Grid item xs={12} sm={6}>
            <Editor
              value={this.state.source}
              keyboardHandler={this.state.keyboardHandler}
              onChange={value => this.handleEditorUpdate(value)}
              onEvaluation={() => this.handleEvaluation()}
            />
          </Grid>

          <Grid item xs={12} sm={6}>
            <Result value={this.state.result} />
          </Grid>
        </Grid>
      </React.Fragment>
    )
  }
}

export default withStyles(styles)(App)
// vim: set ts=2 sw=2 et:
