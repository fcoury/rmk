use crate::keyboard_config::{MATRIX_COLS, MATRIX_ROWS};
use crate::keycodes::Keycode;

pub const NUM_LAYERS: usize = 2; // Example: 2 layers

pub struct Layers {
    keymaps: [[[Keycode; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS],
    current_layer: usize,
}

impl Layers {
    pub fn new() -> Self {
        // Initialize keymaps with default values or load from configuration
        let keymaps = [
            // Layer 0
            [
                [
                    Keycode::A,
                    Keycode::B,
                    Keycode::C,
                    Keycode::D,
                    Keycode::E,
                    Keycode::F,
                    Keycode::G,
                    Keycode::H,
                ],
                [
                    Keycode::I,
                    Keycode::J,
                    Keycode::K,
                    Keycode::L,
                    Keycode::M,
                    Keycode::N,
                    Keycode::O,
                    Keycode::P,
                ],
                [
                    Keycode::Q,
                    Keycode::R,
                    Keycode::S,
                    Keycode::T,
                    Keycode::U,
                    Keycode::V,
                    Keycode::W,
                    Keycode::X,
                ],
                [
                    Keycode::Y,
                    Keycode::Z,
                    Keycode::N1,
                    Keycode::N2,
                    Keycode::N3,
                    Keycode::N4,
                    Keycode::N5,
                    Keycode::N6,
                ],
            ],
            // Layer 1 (Example: Function layer)
            [
                [
                    Keycode::F1,
                    Keycode::F2,
                    Keycode::F3,
                    Keycode::F4,
                    Keycode::F5,
                    Keycode::F6,
                    Keycode::F7,
                    Keycode::F8,
                ],
                [
                    Keycode::F9,
                    Keycode::F10,
                    Keycode::F11,
                    Keycode::F12,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                ],
                [
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                ],
                [
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                    Keycode::No,
                ],
            ],
        ];

        Layers {
            keymaps,
            current_layer: 0,
        }
    }

    pub fn get_keycode(&self, layer: usize, row: usize, col: usize) -> Keycode {
        self.keymaps[layer][row][col]
    }

    pub fn get_current_layer(&self) -> usize {
        self.current_layer
    }

    // Add methods to change layers, handle layer shift keys, etc.
    // Example:
    pub fn set_layer(&mut self, layer: usize) {
        if layer < NUM_LAYERS {
            self.current_layer = layer;
        }
    }
}
