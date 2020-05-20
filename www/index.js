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
    // Draw using Rust
    DLAFieldRenders.draw(field, "dla-display-1")
    // DLAFieldRenders.draw(field, "dla-display-2")

    // Draw using JS
    draw(field, "dla-display-2")

    if (field.next_state()) {
      requestAnimationFrame(renderLoop)
    }
  })
}

requestAnimationFrame(renderLoop)


// temp test state progression
// fields.forEach(field => {
//   // Draw using Rust
//   //DLAFieldRenders.draw(field, "dla-display-1")
//   // DLAFieldRenders.draw(field, "dla-display-2")
//
//   // Draw using JS
//   draw(field, "dla-display-2")
//
//
//   field.next_state();
//   field.next_state();
//   field.next_state();
//   field.next_state();
//   field.next_state();
//   field.next_state();
//
//   draw(field, "dla-display-2")
//
//
// })


function draw(field, canvasId) {
  const canvas = document.getElementById(canvasId);
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "#FF0000";


  const width = field.get_width();
  const height = field.get_height();
  const numAgents = field.get_num_agents();

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  for (let i = 0; i < numAgents; i++) {
    const agent = field.get_agent_at(i);
    const agentX = agent.get_x();
    const agentY = agent.get_y();

    ctx.fillRect(agentX, agentY, 1, 1);
  }
}