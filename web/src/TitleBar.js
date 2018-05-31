import React from 'react'
import { AppBar, Toolbar, Typography } from '@material-ui/core'

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
