use arduino_hal::{
    hal::port::Dynamic,
    port::{
        mode::{Input, Output, PullUp},
        Pin,
    },
    Pins,
};

use crate::keyboard_config::{MATRIX_COLS, MATRIX_ROWS};

pub struct Matrix {
    pub last_state: [[bool; MATRIX_COLS]; MATRIX_ROWS],
    rows: [Pin<Output, Dynamic>; MATRIX_ROWS],
    cols: [Pin<Input<PullUp>, Dynamic>; MATRIX_COLS],
}

impl Matrix {
    pub fn new(pins: Pins) -> Self {
        // Now accepts MatrixPins
        Matrix {
            last_state: [[false; MATRIX_COLS]; MATRIX_ROWS],
            rows: [
                pins.d0.into_output().downgrade(),
                pins.d1.into_output().downgrade(),
                pins.d2.into_output().downgrade(),
                pins.d3.into_output().downgrade(),
            ],
            cols: [
                pins.d4.into_pull_up_input().downgrade(),
                pins.d5.into_pull_up_input().downgrade(),
                pins.d6.into_pull_up_input().downgrade(),
                pins.d7.into_pull_up_input().downgrade(),
                pins.d8.into_pull_up_input().downgrade(),
                pins.d9.into_pull_up_input().downgrade(),
                pins.d10.into_pull_up_input().downgrade(),
                pins.d11.into_pull_up_input().downgrade(), 
            ],
        }
    }

    pub fn scan(&mut self) -> [[bool; MATRIX_COLS]; MATRIX_ROWS] {
        let mut new_state = [[false; MATRIX_COLS]; MATRIX_ROWS];

        for (row_idx, row_pin) in self.rows.iter_mut().enumerate() {
            row_pin.set_low();

            for (col_idx, col_pin) in self.cols.iter().enumerate() {
                new_state[row_idx][col_idx] = col_pin.is_low();
            }

            row_pin.set_high();
        }

        new_state
    }
}
