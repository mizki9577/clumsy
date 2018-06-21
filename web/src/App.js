import React from 'react'

import CssBaseline from '@material-ui/core/CssBaseline'
import Grid from '@material-ui/core/Grid'
import Paper from '@material-ui/core/Paper'
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

const initial_source = String.raw`// Arithmetic operations
let mul = \m n f. m (n f);
let pred = \n f x. n (\g h. h (g f)) (\u. x) (\u. u);
let sub = \m n. n pred m;

// Boolean values
let true = \x y. x;
let false = \x y. y;

// Predicates
let and = \p q. p q p;
let cond = \p then else. p then else;
let is_zero = \n. n (\x. false) true;
let is_equal = \m n. and (is_zero (sub m n)) (is_zero (sub n m));

// Fixed point combinator
let Y = \f. (\x. f (x x)) (\x. f (x x));

// Factorial function
let factorial_impl = \f n.
    cond (is_zero n)
        1
        (mul n (f (pred n)));
let factorial = Y factorial_impl;

// Go!
is_equal (factorial 3) 6;`

// vim: set ts=4 sw=4 et:

class App extends React.Component {
  constructor(props) {
    super(props)

    this.state = {
      source: localStorage.getItem('clumsy.source') || initial_source,
      keyboardHandler: localStorage.getItem('clumsy.keyboardHandler'),
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
    localStorage.setItem('clumsy.keyboardHandler', state.keyboardHandler)
  }

  handleEditorUpdate(source) {
    this.setState({ source })
    localStorage.setItem('clumsy.source', source)
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
