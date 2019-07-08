import { DLAField, FieldPosition } from '../pkg'
import { memory } from '../pkg/wasm_rust_dla_bg'

console.log('!!! much here')
// const field = DLAField.new(600000, 1920, 1920)
const field = DLAField.new(60000, 900, 900)
const width = field.getWidth()
const height = field.getHeight()

const canvas = document.getElementById("dla-display")
canvas.height = height
canvas.width = width


const renderLoop = () => {
  field.draw()

  if (!field.nextState()) {
    requestAnimationFrame(renderLoop)
  }
}

requestAnimationFrame(renderLoop)
