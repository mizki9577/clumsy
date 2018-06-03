import React from 'react'
import AppBar from '@material-ui/core/AppBar'
import Toolbar from '@material-ui/core/Toolbar'
import Typography from '@material-ui/core/Typography'

const TitleBar = () => (
  <AppBar position="static">
    <Toolbar>
      <Typography variant="title" color="inherit">
        Clumsy
      </Typography>
    </Toolbar>
  </AppBar>
)

export default TitleBar
// vim: set ts=2 sw=2 et:
