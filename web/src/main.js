import React from 'react'
import ReactDOM from 'react-dom'
import ClumsyWeb from './ClumsyWeb.js'

import('./clumsy_web.js').then(wasm => {
  ReactDOM.render(<ClumsyWeb wasm={wasm} />, document.querySelector('main'))
})

// vim: set ts=2 sw=2 et:
