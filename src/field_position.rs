use wasm_bindgen::prelude::*;
use crate::colorized_point::ColorizedPoint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldState {
    EMPTY,
    OCCUPIED,
    STUCK
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldPosition {
    pub state: FieldState,
    pub agent: Option<ColorizedPoint>,
    color: [u8; 4]
}

impl FieldPosition {
    pub fn new(state: FieldState, agent: Option<ColorizedPoint>) -> FieldPosition {
        FieldPosition {
            state,
            agent,
            color: [255, 0, 0, 25]
        }
    }
}
