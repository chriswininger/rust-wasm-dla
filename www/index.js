import { DLAField, FieldPosition } from '../pkg'
import { memory } from '../pkg/wasm_rust_dla_bg'


const width = 500;
const height = 500;

const canvases = [
  document.getElementById("dla-display-1"),
  document.getElementById("dla-display-2")
]

const fields = [
  DLAField.new("dla-display-1", 60000, width, height),
  //DLAField.new("dla-display-2", 60000, width, height)
];


canvases.forEach(canvas => {
  canvas.height = height
  canvas.width = width
})

// const width = field.getWidth()
// const height = field.getHeight()


const renderLoop = () => {
  fields.forEach(field =>{
    field.draw()

    if (!field.nextState()) {
      requestAnimationFrame(renderLoop)
    }
  })
}

requestAnimationFrame(renderLoop)
