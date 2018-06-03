import React from 'react'
import Button from '@material-ui/core/Button'
import FormControl from '@material-ui/core/FormControl'
import Grid from '@material-ui/core/Grid'
import InputLabel from '@material-ui/core/InputLabel'
import MenuItem from '@material-ui/core/MenuItem'
import Select from '@material-ui/core/Select'

const MenuBar = ({ className, keyboardHandler, onChange, onEvaluation }) => (
  <Grid
    container
    justify="space-between"
    spacing={16}
    alignItems="flex-end"
    className={className}
  >
    <Grid item>
      <FormControl>
        <InputLabel>Editor&nbsp;Mode</InputLabel>
        <Select
          value={keyboardHandler || 'default'}
          onChange={ev =>
            onChange({
              keyboardHandler:
                ev.target.value === 'default' ? null : ev.target.value,
            })
          }
        >
          <MenuItem value="default">Default</MenuItem>
          <MenuItem value="vim">Vim</MenuItem>
          <MenuItem value="emacs">Emacs</MenuItem>
        </Select>
      </FormControl>
    </Grid>

    <Grid item>
      <Button variant="raised" color="primary" onClick={() => onEvaluation()}>
        Evaluate
      </Button>
    </Grid>
  </Grid>
)

export default MenuBar
// vim: set ts=2 sw=2 et:
