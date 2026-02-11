// Level - Microbit v2 implements digital level

use crate::level::bubble::Bubble;

const CENTER_ROW_OFFSET: i8 = 2;
const CENTER_COL_OFFSET: i8 = 2;
const BUBBLE_MAX_ROW: i8 = 4;
const BUBBLE_MAX_COL: i8 = 4;
// rustc --explain E0616
mod bubble {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Bubble {
        pub row: i8,
        pub col: i8,
    }

    impl Bubble 
    {
        pub fn new() -> Bubble { Bubble { row: 0_i8, col: 0_i8 } }
    }
}

// Implement a data type for the life 5x5 "field"

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct DisplayData {
    led_array: [[u8; 5]; 5],
}

impl DisplayData {
    fn new() -> Self {
        let led_array = [[0; 5]; 5];
        Self {
            led_array,
        }
    }

    fn clear(&mut self) -> [[u8; 5]; 5] {
        for row in 0..5 {
            for col in 0..5 {
                self.led_array[row][col] = 0;
            }
        }
        self.led_array
    }

    fn pixel_on(&mut self, row: i8, col: i8) -> [[u8; 5]; 5] {
        for r in 0..=BUBBLE_MAX_ROW {
            for c in 0..=BUBBLE_MAX_COL {
                if r == row && c == col {
                    self.led_array[r as usize][c as usize] = 1;
                }
            }
        }
        self.led_array
    }

}

pub enum SenseResolution {
    Course,
    Fine
}

#[derive(Debug, Copy, Clone)]
pub enum ButtonPress {
    ButtonA,
    ButtonB,
    None
}

// Struct "Level" describes the shape of data associated with the
// run-time level app

pub(crate) struct Level {
    display_data: DisplayData,
    bubble: Bubble,
    sense_mode: SenseResolution,
    // flag_initialized: bool
}

impl Level {

    pub(crate) fn new() -> Self {
        let mut display_data = DisplayData::new();
        display_data.clear();
        let bubble = Bubble { row: CENTER_ROW_OFFSET, col: CENTER_COL_OFFSET };
        Self {
            display_data,
            bubble,
            sense_mode: SenseResolution::Course,
        }
    }

    pub(crate) fn bubble_x_y(&self) -> Bubble {
        self.bubble
    }

    pub(crate) fn sense_mode_c_f(&self) -> u8 {
        let mut mode: u8 = 0;
        match self.sense_mode { 
            SenseResolution::Course => {mode = 1},
            SenseResolution::Fine => {mode = 2},
        }
        mode
    }

    pub(crate) fn current_render(
        &self,
    ) -> [[u8; 5]; 5] {
        self.display_data.led_array
    }

    pub(crate) fn pixel_on(&mut self, row: i8, col: i8) -> [[u8; 5]; 5] {
        self.display_data.clear();
        let mut new_row = row + CENTER_ROW_OFFSET;
        let mut new_col = col + CENTER_COL_OFFSET;
        match new_row {
            i8::MIN..=0 => new_row = 0,
            BUBBLE_MAX_ROW..=i8::MAX => new_row = BUBBLE_MAX_ROW,
            _ => new_row = row + CENTER_ROW_OFFSET,
        }
        match new_col {
            i8::MIN..=0 => new_col = 0,
            BUBBLE_MAX_COL..=i8::MAX => new_col = BUBBLE_MAX_COL,
            _ => new_col = col + CENTER_COL_OFFSET,
        }
        self.bubble.row = new_row;
        self.bubble.col = new_col;

        self.display_data.pixel_on(new_row, new_col);
        self.display_data.led_array
    }

    fn handle_button_A(&mut self) {
        self.sense_mode = SenseResolution::Course;
    }

    fn handle_button_B(&mut self) {
        self.sense_mode = SenseResolution::Fine;
    }

    pub(crate) fn handle_buttons(&mut self, binput: ButtonPress) {
        match binput {
            ButtonPress::ButtonA => { self.handle_button_A() },
            ButtonPress::ButtonB => { self.handle_button_B() },
            // ButtonPress::None => { self.handle_buttons_released() }
            ButtonPress::None => { }
        }
    }

}
