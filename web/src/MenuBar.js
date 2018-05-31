import React from 'react'
import {
  Button,
  FormControl,
  Grid,
  InputLabel,
  MenuItem,
  Select,
} from '@material-ui/core'

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
