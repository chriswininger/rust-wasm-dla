use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StickyNeighbor {
    pub x: usize,
    pub y: usize
}

#[wasm_bindgen]
impl StickyNeighbor {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> StickyNeighbor {
        StickyNeighbor {x, y}
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    FREE,
    STUCK
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[wasm_bindgen]
impl Color {

    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_g(&self) -> u8 {
        self.g
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorizedPoint {
    pub x: usize,
    pub y: usize,
    pub state: AgentState,
    pub sticky_neighbor: Option<StickyNeighbor>,
    color: Color
}

#[wasm_bindgen]
impl ColorizedPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize, color: Color, sticky_neighbor: Option<StickyNeighbor>) -> ColorizedPoint {
        ColorizedPoint {
            x,
            y,
            color,
            state: AgentState::FREE,
            sticky_neighbor
        }
    }

    pub fn get_agent_state(&self) -> AgentState {
        self.state
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_sticky_neighbor(&self) -> Option<StickyNeighbor> {
        self.sticky_neighbor
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}
