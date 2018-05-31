import React from 'react'
import { Grid } from '@material-ui/core'
import AceEditor from 'react-ace'

class ClumsyWeb extends React.Component {
  constructor(props) {
    super(props)

    this.state = {
      source: '',
    }
  }

  render() {
    return (
      <Grid container>
        <Grid item xs={12} sm={6}>
          <Editor value={this.state.source} onChange={ source=>{ this.setState({ source }) } } />
        </Grid>

        <Grid item xs={12} sm={6}>
          <Result value={this.state.source} />
        </Grid>
      </Grid>
    )
  }
}

class Editor extends React.Component {
  render() {
    return (
      <AceEditor value={this.props.value} onChange={ (value, _) => this.props.onChange(value) } />
    )
  }
}

class Result extends React.Component {
  render() {
    return (
      <div>
        {this.props.value}
      </div>
    )
  }
}

export default ClumsyWeb
// vim: set ts=2 sw=2 et:
