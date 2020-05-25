#[macro_use]
extern crate serde_derive;

extern crate js_sys;
extern crate web_sys;

mod utils;
mod colorized_point;
mod field_position;

use wasm_bindgen::prelude::*;
use js_sys::{Math, Array, ArrayIter};
use web_sys::*;
use wasm_bindgen::convert::{FromWasmAbi, WasmAbi};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::colorized_point::*;

use crate::field_position::FieldPosition;
use crate::field_position::FieldState;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DLAField {
    width: usize,
    height: usize,
    canvas_id: String,
    agents: Vec<ColorizedPoint>,
    agent_position_lookup: Vec<Vec<Option<usize>>>,
    position_hash: Vec<FieldPosition>
}

// === Static Methods ===
#[wasm_bindgen]
impl DLAField {

    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: String, num_agents: usize, width: usize, height: usize) -> DLAField {
        let mut position_hash = DLAField::generateEmptyPositionHash(width, height);
        let mut agent_position_lookup = vec![vec![None; height]; width];
        let mut agents: Vec<ColorizedPoint> = [].to_vec();

        for i in 0..num_agents {
            let mut x = DLAField::gen_range(0, width);
            let mut y =  DLAField::gen_range(0, height);

            while DLAField::isPositionOccupied(&position_hash, x, y, width) {
                x =  DLAField::gen_range(0, width);
                y =  DLAField::gen_range(0, height);
            }

            let agent = ColorizedPoint::new(
                x, y, Color::new(255, 0, 0, 100), None);

            let ndx = DLAField::get_ndx(x, y, width);

            // occupy the position
            position_hash[ndx] = FieldPosition::new(FieldState::OCCUPIED, Some(agent));

            // store the agent
            agent_position_lookup[agent.get_x()][agent.get_y()] = Some(agents.len());
            agents.push(agent);
        }

        DLAField {
            width,
            height,
            agents,
            position_hash,
            canvas_id,
            agent_position_lookup
        }
    }

    fn generateEmptyPositionHash(width: usize, height: usize) -> Vec<FieldPosition> {
        (0..width * height).map(|ndx| {
            FieldPosition::new(FieldState::EMPTY, None)
        }).collect()
    }

    fn isPositionOccupied(positionHash: &Vec<FieldPosition>, x: usize, y: usize, width: usize) -> bool {
        let ndx = DLAField::get_ndx(x, y, width);

        let val = &positionHash[ndx];
        match val.state {
            FieldState::OCCUPIED => true,
            FieldState::STUCK => true,
            FieldState::EMPTY => false
        }
    }

    fn gen_range(min: usize, max: usize) -> usize {
        let castMin = min as f64;
        let castMax = max as f64;

        let rnd: f64 = Math::random();
        Math::floor(rnd * (castMax - castMin) + castMin) as usize
    }

    pub fn get_ndx(x: usize, y: usize, width: usize) -> usize {
        x * width + y
    }

    fn gen_bool(prob: f64) -> bool {
        // console::log_1(&"gen_bool - 1".into());
        let coin: f64 = Math::random();

        // console::log_2(&"gen_bool - 2".into(), &coin.into());
        if coin <= prob {
            // console::log_1(&"gen_bool - 3".into());
            return true;
        } else {
            // console::log_1(&"gen_bool - 4".into());
            return false;
        }
    }
}

// === Instance Methods ===

#[wasm_bindgen]
impl DLAField {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.width
    }

    pub fn get_num_agents(&self) -> usize {
        self.agents.len()
    }

    pub fn get_agent_at(&self, ndx: usize) -> ColorizedPoint {
        self.agents[ndx]
    }

    fn get_agent_at_borrow(&mut self, ndx: usize) -> &mut ColorizedPoint {
        &mut self.agents[ndx]
    }

    pub fn next_state(&mut self) -> bool {
        let mut has_next_state = false;

        // might be better to walk the y array in reverse so we check/update lowest first
        let mut cntStuck = 0;

        let mut new_agents: Vec<ColorizedPoint> = [].to_vec();
        let mut new_agent_position_lookup: Vec<Vec<Option<usize>>> = vec![vec![None; self.get_height()]; self.get_width()];

        for x in 0..self.width {
            for y in (0..self.height).rev() {
                let field_ndx = DLAField::get_ndx(x, y, self.width);

                let agent_at_position = self.get_agent_at_coordinate(x, y);

                match agent_at_position {
                    Some(mut agent) => {
                        match agent.state {
                            AgentState::FREE => {
                                has_next_state = true;

                                let stuck = self.is_stuck(x, y, false);

                                if stuck.0 {
                                    match stuck.1 {
                                        None => {
                                            agent.state = AgentState::STUCK;
                                            agent.sticky_neighbor = None
                                        },
                                        Some(neighbor_position) => {
                                            // set to stuck along with the position of the neighbor that caused it to stick
                                            agent.state = AgentState::STUCK;
                                            agent.sticky_neighbor = Some(StickyNeighbor {
                                                x: neighbor_position.0,
                                                y: neighbor_position.1
                                            });

                                        }
                                    }

                                    self.position_hash[field_ndx] =
                                        FieldPosition::new(FieldState::STUCK, Some(agent));
                                    // console::log_1(&"!!! should change to stuck for reals".into());

                                } else {
                                    // find the next available position
                                    let new_position = self.findNextPosition(x, y);

                                    // check that we didn't just resolve the same location
                                    if x != new_position.0 && y != new_position.1 {
                                        self.move_position(
                                            &mut agent,
                                            new_position.0,
                                            new_position.1,
                                            new_agents.len()
                                        );
                                    }
                                }
                            },
                            AgentState::STUCK => {}
                        }

                        // update new vector index system
                        new_agent_position_lookup[agent.get_x()][agent.get_y()] = Some(new_agents.len());
                        new_agents.push(agent);
                    }
                    None => {}
                }
            }
        }

        self.agents = new_agents;
        self.agent_position_lookup = new_agent_position_lookup;

        has_next_state
    }

    fn find_agent_at_coordinate(&self, x: usize, y: usize) -> Option<&ColorizedPoint> {
        self.agent_iterator()
            .find(|agent| agent.get_x() == x && agent.get_y() == y)
    }

    fn findNextPosition(&self, x: usize, y: usize) -> (usize, usize) {
        // needs to be i32 to prevent overflow
        let x = x as i32;
        let y = y as i32;

        // console::log_1(&"nextState - 0".into());
        let mut newX = if  DLAField::gen_bool(0.5) { x + 1 } else { x - 1 };
        let mut newY = if  DLAField::gen_bool(0.75) { y + 1 } else { y - 1 };

        // console::log_3(&"findNexPos 1".into(), &newX.into(), &newY.into());
        let width = self.get_width() as i32;
        let height = self.get_height() as i32;

        // TOOD (CAW): Consider pre-calculating available states and if there is just one possibility take it
        while newX < 0 || newY < 0 || newX >= width || newY >= height {
            newX = if  DLAField::gen_bool(0.5) { x + 1 } else { x - 1 };
            newY = if  DLAField::gen_bool(0.75) { y + 1 } else { y - 1 };
        }

        let mut attemptCount = 0;

        while  DLAField::isPositionOccupied(&self.position_hash, newX as usize, newY as usize, self.width) && attemptCount <= 4 {
            while newX < 0 || newY < 0 || newX >= width || newY >= height {
                newX = if  DLAField::gen_bool(0.5) { x + 1 } else { x - 1 };
                newY = if  DLAField::gen_bool(0.75) { y + 1 } else { y - 1 };
            }

            attemptCount += 1;
        }

        // console::log_1(&"finedNextPositoin -- end".into());
        if attemptCount < 4 {
            return (newX as usize, newY as usize)
        } else {
            return (x as usize, y as usize)
        }
    }

    fn agent_iterator(&self) -> std::slice::Iter<ColorizedPoint> {
        self.agents.iter()
    }

    fn move_position(
        &mut self,
        agent: &mut ColorizedPoint,
        new_x: usize,
        new_y: usize,
        new_agent_ndx: usize
    ) {
        let x = agent.get_x();
        let y= agent.get_y();

        let old_field_ndx = DLAField::get_ndx(x, y, self.get_width());
        let new_field_ndx = DLAField::get_ndx(
            new_x, new_y, self.get_width());

        // update the agent
        agent.set_x(new_x);
        agent.set_y(new_y);

        // update old position_hash system
        self.position_hash[old_field_ndx] =
            FieldPosition::new(FieldState::EMPTY, None);

        self.position_hash[new_field_ndx] =
            FieldPosition::new(FieldState::OCCUPIED, Some(*agent));
    }

    fn is_stuck(&self, _x: usize, _y: usize, recursion: bool) -> (bool, Option<(usize, usize)>) {
        let width = self.get_width() as i32;
        let height = self.get_height() as i32;
        let x = _x as i32;
        let y = _y as i32;

        let agent = self.get_agent_at_coordinate(_x, _y);
        match agent {
            None => return (false, None),
            Some(agent) => {
                let agent_state = agent.get_agent_state();
                match agent_state {
                    AgentState::STUCK => {
                        return match agent.sticky_neighbor {
                            None => return (true, None), // already stuck, no neighbor
                            Some(position) => {
                                // already stuck pass along it's stuck neighbor if applicable
                                return (true, Some((position.x, position.y)))
                            }
                        }
                    },
                    AgentState::FREE => {
                        if y >= height - 1 {
                            return (true, None); // stuck, this is the root, no neighbor
                        }
                    }
                }
            }
        }

        if recursion {
            return  (false, None);
        }

        let mut dx: i32 = -1;
        if x == 0 {
            dx = 0;
        }

        while dx < 2 {
            let mut dy: i32 = -1;

            if y == 0 {
                dy = 0;
            }

            while dy < 2 {
                let neighbor_x = x as i32 + dx as i32;
                let neighbor_y = y as i32 + dy as i32;

                if neighbor_x < width && neighbor_y < height &&
                    self.is_stuck(neighbor_x as usize, neighbor_y as usize, true).0
                {
                    // stuck with a neighbor
                    return (true, Some((neighbor_x as usize, neighbor_y as usize)))
                }

                dy += 1;
            }

            dx += 1
        }

        (false, None)
    }


    fn _get_distance_from_root(&self, agent: ColorizedPoint, size: usize) -> usize {
        match agent.get_sticky_neighbor() {
            None => {
                0
            },
            Some(neighbor_position) => {
                let neighbor_x = neighbor_position.x;
                let neighbor_y = neighbor_position.y;

                let neighbor = self.get_agent_at_coordinate(neighbor_x, neighbor_y);


                // assume there is a neighbor based on neighbor_position, if not panic
                size + self._get_distance_from_root(neighbor.unwrap(), size + 1)
            }
        }
    }

    pub fn get_distance_from_root(&self, agent: ColorizedPoint) -> usize {
        self._get_distance_from_root(agent, 0)
    }

    fn get_agent_at_coordinate(&self, x: usize, y: usize) -> Option<ColorizedPoint> {
        match self.agent_position_lookup[x][y] {
            None => { None }
            Some(ndx) => {
                Some(self.get_agent_at(ndx))
            }

        }
    }

    fn isEmpty(&self, x: u32, y: u32) -> bool {
        let ndx = DLAField::get_ndx(x as usize, y as usize, self.get_width());
        if let FieldState::EMPTY = self.position_hash[ndx].state {
            return true
        }

        false
    }

    // this is more for testing
    pub fn getOccpupiedCount(&self) -> u32 {
        let mut cnt = 0;

        for ndx in 0..self.position_hash.len() {
            match &self.position_hash[ndx].state {
                FieldState::EMPTY => {},
                FieldState::OCCUPIED => {
                    cnt += 1;
                },
                FieldState::STUCK => {
                    cnt += 1;
                }
            }
        }

        cnt
    }

    pub fn get_position_hash(&self) -> *const FieldPosition {
        self.position_hash.as_ptr()
    }

    // this is more for testing
    pub fn getStuckCount(&self) -> u32 {
        let mut cnt = 0;

        for ndx in 0..self.position_hash.len() {
            match &self.position_hash[ndx].state {
                FieldState::EMPTY => {},
                FieldState::OCCUPIED => {},
                FieldState::STUCK => {
                    cnt +=  1;
                }
            }
        }

        cnt
    }
}

#[wasm_bindgen]
pub struct DLAFieldRenders {}

#[wasm_bindgen]
impl DLAFieldRenders {
    pub fn draw(dla_field: &DLAField, canvas_id: String) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).unwrap();
        let stuckSize = 1.0;
        let seedSize = 1.0;

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let width = canvas.width();
        let height = canvas.height();

        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        for ndx in 0..dla_field.position_hash.len() as u32 {
            let y = (ndx % width);
            let x = (ndx / height);

            let position: FieldPosition = dla_field.position_hash[ndx as usize];
            match position.state {
                FieldState::EMPTY => {},
                FieldState::STUCK => {
                    context.set_fill_style(&"rgba(255, 0, 0, 255)".into());
                    context.fill_rect(x as f64 * stuckSize, y as f64 * stuckSize, stuckSize, stuckSize);
                },
                FieldState::OCCUPIED => {
                    context.set_fill_style(&"rgba(255, 0, 0, 255)".into());
                    context.fill_rect(x as f64 * seedSize, y as f64 * seedSize, seedSize, seedSize);
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn build_field_from_js_state(canvas_id: String, width: usize, height: usize, agents: Array ) -> DLAField {
    console::log_1(&"initialize a new field".into());
    let mut new_field = DLAField::new(canvas_id, agents.length() as usize, width, height);

    // even this fails with a runtime exception, much boo :-(
    // let test: ColorizedPoint = agents.get(0).into_serde().unwrap();

    console::log_1(&"deserialize agents".into());
    let agents_vec : Vec<ColorizedPoint> = agents.into_serde().unwrap();

    console::log_1(&"clone incoming to the agents field".into());
    new_field.agents = agents_vec.clone();

    // build the location based lookup table
    let mut agent_position_lookup: Vec<Vec<Option<usize>>> = vec![vec![None; height]; width];

    console::log_1(&"build lookup table".into());
    for i in 0..new_field.get_num_agents() {
        let agent = new_field.get_agent_at(i);
        let x = agent.get_x();
        let y = agent.get_y();

        agent_position_lookup[x][y] = Some(i);
    }

    console::log_1(&"return final value".into());
    new_field
}
