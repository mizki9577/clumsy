import React from 'react'
import Paper from '@material-ui/core/Paper'
import { withStyles } from '@material-ui/core/styles'

const styles = theme => ({
  result: {
    fontFamily: 'monospace',
  },
})

const Result = ({ value, classes }) => (
  <Paper className={classes.result}>{value}</Paper>
)

export default withStyles(styles)(Result)
// vim: set ts=2 sw=2 et:
