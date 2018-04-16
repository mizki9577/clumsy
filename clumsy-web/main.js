import React from 'react'
import ReactDOM from 'react-dom'
import Terminal from 'terminal-in-react'
import './index.html'
import './style.css'

const ClumsyWeb = () => (
  <Terminal />
)

ReactDOM.render(
  <ClumsyWeb />,
  document.querySelector('main'),
)

// vim: set ts=2 sw=2 et:
