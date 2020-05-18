import {ColorizedPoint, DLAField, DLAFieldRenders, FieldOfPlay, FieldPosition} from '../pkg'
import {colorizedpoint_new, memory} from '../pkg/wasm_rust_dla_bg'


const width = 500;
const height = 500;

const canvases = [
  document.getElementById("dla-display-1"),
  document.getElementById("dla-display-2")
]

// const pointTest = new ColorizedPoint(51, 62);
// const parseColorArray = (pointer) => {
//   return new Uint8Array(memory.buffer, pointer, 4)
// }

// const fieldOfPlay = new FieldOfPlay(500, 500, 1000);
//
// console.log('!!! fieldOfPlay: ' + fieldOfPlay.getHeight() + ", " + fieldOfPlay.get_agent_at(0).get_x());
//
// console.log('!!! pointTest: ' + JSON.stringify(pointTest, null, 4));
// console.log('!!! and then: ' + pointTest.get_x() + ", " + pointTest.get_y());
// console.log('!!! colors: ' + JSON.stringify(parseColorArray(pointTest.get_color_pointer())));



const fields = [
  new DLAField("dla-display-1", 60000, width, height)
  //new FieldOfPlay("dla-display-1", 60000, width, height),
  //DLAField.new("dla-display-2", 60000, width, height)
];


canvases.forEach(canvas => {
  canvas.height = height
  canvas.width = width
})

// const width = field.getWidth()
// const height = field.getHeight()


const renderLoop = () => {
  fields.forEach(field => {
    DLAFieldRenders.draw(field, "dla-display-1")

    if (!field.nextState()) {
      requestAnimationFrame(renderLoop)
    }
  })
}

requestAnimationFrame(renderLoop)
