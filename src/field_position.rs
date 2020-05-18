use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum FieldState {
    EMPTY,
    OCCUPIED,
    STUCK
}

#[derive(Clone, Copy, Debug)]
pub struct FieldPosition {
    pub state: FieldState,
    color: [u8; 4]
}

impl FieldPosition {
    pub fn new(state: FieldState) -> FieldPosition {
        FieldPosition {
            state,
            color: [255, 0, 0, 25]
        }
    }
}
