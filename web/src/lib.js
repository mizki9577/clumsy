import lib from './lib.rs'

export const evaluate = source => {
  const te = new TextEncoder()
  const td = new TextDecoder()

  const source_array = te.encode(source)
  const source_size = source_array.length

  const destination_size = source_size + 1
  const destination_ptr = lib.alloc(destination_size)
  const destination_array = new Int8Array(lib.memory.buffer, destination_ptr, destination_size)
  destination_array.set(source_array)
  destination_array[destination_size - 1] = 0  // terminating by null

  const result_ptr = lib.eval(destination_ptr)
  const result_array = new Uint8Array(lib.memory.buffer, result_ptr)

  let i = 0
  while (result_array[i] != 0) {
    ++i
  }
  const result = td.decode(result_array.slice(0, i))

  lib.dealloc(destination_ptr, destination_size)
  lib.free_result(result_ptr)

  return result
}

// vim: set ts=2 sw=2 et:
