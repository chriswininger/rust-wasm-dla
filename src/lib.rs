extern crate js_sys;
extern crate web_sys;

mod utils;
mod colorized_point;
mod field_position;

use wasm_bindgen::prelude::*;
use js_sys::Math;
use web_sys::*;
use wasm_bindgen::convert::{FromWasmAbi, WasmAbi};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::colorized_point::ColorizedPoint;
use crate::colorized_point::Color;
use crate::field_position::FieldPosition;
use crate::field_position::FieldState;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-wasm-dla!");
}

#[wasm_bindgen]
pub struct DLAField {
    width: usize,
    height: usize,
    canvas_id: String,
    agents: Vec<ColorizedPoint>,
    position_hash: Vec<FieldPosition>
}

// === Static Methods ===

#[wasm_bindgen]
impl DLAField {

    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: String, num_agents: usize, width: usize, height: usize) -> DLAField {
        let mut position_hash = DLAField::generateEmptyPositionHash(width, height);
        let mut agents: Vec<ColorizedPoint> = [].to_vec();

        for i in 0..num_agents {
            let mut x = DLAField::gen_range(0, width);
            let mut y =  DLAField::gen_range(0, height);

            while DLAField::isPositionOccupied(&position_hash, x, y, width) {
                x =  DLAField::gen_range(0, width);
                y =  DLAField::gen_range(0, height);
            }

            let ndx = DLAField::get_ndx(x, y, width);

            // occupy the position
            position_hash[ndx] = FieldPosition::new(FieldState::OCCUPIED);

            // store the agent
            agents.push(ColorizedPoint::new(x, y, Color::new(255, 0, 0, 100)));
        }

        DLAField {
            width,
            height,
            agents,
            position_hash,
            canvas_id
        }
    }

    fn generateEmptyPositionHash(width: usize, height: usize) -> Vec<FieldPosition> {
        (0..width * height).map(|ndx| {
            FieldPosition::new(FieldState::EMPTY)
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
    pub fn getWidth(&self) -> usize {
        self.width
    }

    pub fn getHeight(&self) -> usize {
        self.width
    }

    pub fn get_agent_at(&self, ndx: usize) -> ColorizedPoint {
        self.agents[ndx]
    }

    pub fn nextState(&mut self) -> bool {
        let mut isDone = true;

        // might be better to walk the y array in reverse so we check/update lowest first
        let mut cntStuck = 0;

        /*
          TODO (CAW) While walking backwords is better perhaps it would be better to have list of (x, y)
          with only the occupied points (like we used to) and only iterate over those, only issue is
          can't go top to bottom so... (if doing this consider a move function(x1, y1, x2, y2) which
          updates both pieces of state
        */
        for x in 0..self.getWidth() {
            // walk y in reverse so points near the bottom get stuck first
            for y in (0..self.getHeight()).rev() {

                let ndx = DLAField::get_ndx(x, y, self.getWidth());

                let stuck = self.isStuck(x, y, false);

                let curVal =  &self.position_hash[ndx];

                if stuck {
                    cntStuck += 1;
                }

                match curVal.state {
                    FieldState::OCCUPIED => {
                        if !stuck {
                            isDone = false;

                            let newPosition = self.findNextPosition(x, y);
                            let newNdx = DLAField::get_ndx(
                                newPosition.0, newPosition.1, self.getWidth());

                            if x != newPosition.0 && y != newPosition.1 {
                                self.position_hash[newNdx] =
                                    FieldPosition::new(FieldState::OCCUPIED);

                                self.position_hash[ndx] =
                                    FieldPosition::new(FieldState::EMPTY);
                            }
                        } else {
                            self.position_hash[ndx] =
                                FieldPosition::new(FieldState::STUCK)
                        }
                    },
                    FieldState::STUCK => {},
                    FieldState::EMPTY => {}
                }
            }
        }

        if cntStuck % 100 == 0 {
            console::log_2(&"cntStuck:".into(), &cntStuck.into());
        }

        isDone
    }

    fn findNextPosition(&self, x: usize, y: usize) -> (usize, usize) {
        // needs to be i32 to prevent overflow
        let x = x as i32;
        let y = y as i32;

        // console::log_1(&"nextState - 0".into());
        let mut newX = if  DLAField::gen_bool(0.5) { x + 1 } else { x - 1 };
        let mut newY = if  DLAField::gen_bool(0.75) { y + 1 } else { y - 1 };

        // console::log_3(&"findNexPos 1".into(), &newX.into(), &newY.into());
        let width = self.getWidth() as i32;
        let height = self.getHeight() as i32;

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

    fn isStuck(&self, _x: usize, _y: usize, recursion: bool) -> bool {
        let width = self.getWidth() as i32;
        let height = self.getHeight() as i32;
        let x = _x as i32;
        let y = _y as i32;

        let ndx = DLAField::get_ndx(x as usize, y as usize, width as usize);

        match &self.position_hash[ndx].state {
            FieldState::EMPTY => {
                return false
            },
            FieldState::STUCK => return true,
            FieldState::OCCUPIED => {
                if y >= height - 1 {
                    return true;
                }
            }
        }

        if recursion {
            return  false;
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
                let neighborX = x as i32 + dx as i32;
                let neighborY = y as i32 + dy as i32;

                if neighborX < width && neighborY < height &&
                    self.isStuck(neighborX as usize, neighborY as usize, true)
                {
                    return true
                }

                dy += 1;
            }

            dx += 1
        }

        false
    }

    fn isEmpty(&self, x: u32, y: u32) -> bool {
        let ndx = DLAField::get_ndx(x as usize, y as usize, self.getWidth());
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

