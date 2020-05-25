import {
  AgentState,
  build_field_from_js_state, Color,
  ColorizedPoint,
  DLAField,
  DLAFieldRenders,
  FieldOfPlay,
  FieldPosition,
  StickyNeighbor
} from '../pkg'
import LZString from 'lz-string'
import {colorizedpoint_new, memory} from '../pkg/wasm_rust_dla_bg'



const canvas_id_1 = "dla-display-1"
const canvas_id_2 = "dla-display-2"
const canvas_id_3 = "dla-display-3"
const width = 100
const height = 100

const canvases = [
  document.getElementById("dla-display-1"),
  document.getElementById("dla-display-2")
]


document.getElementById('btnStartAnimation').onclick = (e) => {
  e.preventDefault()
  requestAnimationFrame(renderLoop)
}

document.getElementById('btnRunToCompleteThenRender').onclick = (e) => {
  e.preventDefault()
  runToCompleteThenRender()
}

document.getElementById('btnRestoreState').onclick = (e) => {
  e.preventDefault()
  getStateFromLocalStorage()
}

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
  new DLAField("dla-display-1", 3000, width, height)
  //new FieldOfPlay("dla-display-1", 60000, width, height),
  //DLAField.new("dla-display-2", 60000, width, height)
];


// we'll probably just have 1
const field = fields[0]

canvases.forEach(canvas => {
  canvas.height = height
  canvas.width = width
})

// const width = field.getWidth()
// const height = field.getHeight()


const renderLoop = () => {
  fields.forEach(field => {
    // Draw using Rust
    DLAFieldRenders.draw(field, canvas_id_1)
    // DLAFieldRenders.draw(field, "dla-display-2")

    // Draw using JS
    draw(field, canvas_id_2)

    drawStickVersion(field, canvas_id_3)

    if (field.next_state()) {
      requestAnimationFrame(renderLoop)
    } else {
      // Completed render
      console.log(`completed render, longest tree: ' ${findTallest(field)}`)

      // save state
      saveStateToLocalStorage(field)
    }
  })
}

function runToCompleteThenRender() {

  while (field.next_state()) {}

  DLAFieldRenders.draw(field, "dla-display-1")
  draw(field, "dla-display-2")

  saveStateToLocalStorage(field)
}

function findTallest(field) {
  let tallest = 0

  const numAgents = field.get_num_agents();

  for (let i = 0; i < numAgents; i++) {
    const agent = field.get_agent_at(i);
    const dist = field.get_distance_from_root(agent)

    if (dist > tallest) {
      tallest = dist
    }
  }

  return tallest
}

function draw(field, canvasId) {
  const longestChain = 2926 // 69378
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
    const stickyNeighbor = agent.get_sticky_neighbor();


    const dist = field.get_distance_from_root(agent)
    const mappedDistance = Math.round(mapFromRangeToRange(dist, 0, longestChain, 0, 255))
    const agentSize =  Math.round(
        mapFromRangeToRange(dist, 0, longestChain, 3, 15) || 1)


    ctx.fillStyle = `rgb(${ 255 - mappedDistance }, ${ mappedDistance }, 0)`

    ctx.fillRect(agentX, agentY, agentSize || 3, agentSize || 3);

    //ctx.fillStyle = "#FF0000";
  }
}

function drawStickVersion(field, canvasId) {
  const multiplier = 8
  const canvas = document.getElementById(canvasId);
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "#FF0000";


  const width = field.get_width() * multiplier
  const height = field.get_height() * multiplier

  canvas.height = height
  canvas.width = width

  const numAgents = field.get_num_agents();

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  for (let i = 0; i < numAgents; i++) {
    const agent = field.get_agent_at(i);
    const agentX = agent.get_x();
    const agentY = agent.get_y();
    const stickyNeighbor = agent.get_sticky_neighbor();


    const dist = field.get_distance_from_root(agent)
    const mappedDistance = Math.round(mapFromRangeToRange(dist, 0, 69378, 0, 255))

    const agentTransformX = agentX * multiplier
    const agentTransformY = agentY * multiplier



    if (stickyNeighbor) {
      const neighborTransformX = stickyNeighbor.x * multiplier
      const neighborTransformY = stickyNeighbor.y * multiplier

      ctx.fillStyle = `rgb(150, 255, 0)`
      ctx.strokeStyle = `rgb(150, 255, 0)`

      ctx.beginPath();
      ctx.arc(agentTransformX, agentTransformY, multiplier/2, 0, 2 * Math.PI);
      ctx.stroke();

      ctx.fillStyle = `rgb(100, 255, 0)`
      ctx.strokeStyle = `rgb(100, 255, 0)`

      ctx.moveTo(agentTransformX, agentTransformY)
      ctx.lineTo(neighborTransformX, neighborTransformY)
      ctx.stroke()
    } else {
      ctx.fillStyle = `rgb(150, 255, 0)`
      ctx.strokeStyle = `rgb(150, 255, 0)`
      ctx.beginPath();
      ctx.arc(agentTransformX, agentTransformY, multiplier/2, 0, 2 * Math.PI);
      ctx.stroke();
    }

    //ctx.fillStyle = "#FF0000";
  }
}

function mapFromRangeToRange(valToMap, range1Start, range1End, range2Start, range2End) {
  return (valToMap - range1Start) / (Math.abs(range1End - range1Start))
      * Math.abs(range2End - range2Start) + range1Start;
}

function saveStateToLocalStorage(field) {
  const stateToSave = []

  const numAgents = field.get_num_agents();

  for (let i = 0; i < numAgents; i++) {
    const agent = field.get_agent_at(i);
    const stickyNeighbor = agent.get_sticky_neighbor();
    const stickyNeighborJS = !stickyNeighbor ? null : {
      x: stickyNeighbor.x,
      y: stickyNeighbor.y
    }

    const jsAgent = {
      x: agent.get_x(),
      y: agent.get_y(),
      state: AgentState.STUCK,
      color: {
        r: agent.get_color().get_r(),
        g: agent.get_color().get_g(),
        b: agent.get_color().get_b(),
        a: agent.get_color().get_a()
      },
      stickyNeighbor: stickyNeighborJS
    }

    stateToSave.push(jsAgent)
  }

  const itemToSave = JSON.stringify(stateToSave)
  const itemToSaveCompressed = LZString.compress(itemToSave)

  console.log(`set it to storage: ${itemToSave.length}`)
  console.log(`compressed len: ${itemToSaveCompressed.length}`)

  localStorage.setItem('last_state', itemToSaveCompressed)

  console.log('done')
}

function getStateFromLocalStorage() {
  const itemStr = localStorage.getItem('last_state')
  const decompressedItemStr = LZString.decompress(itemStr)

  const state = JSON.parse(decompressedItemStr)

  const agents = state.map(item => {
    const x = item.x
    const y = item.y

    const color = new Color(
      item.color.r,
      item.color.g,
      item.color.b,
      item.color.a
    )

    const neighbor = item.stickyNeighbor ? new StickyNeighbor(item.stickyNeighbor.x, item.stickyNeighbor.y) : null

    return new ColorizedPoint(x, y, color, neighbor)
  })

  const newField = build_field_from_js_state(canvas_id_2, width, height, agents)

  draw(newField, canvas_id_2)
}