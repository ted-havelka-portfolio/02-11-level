// Level - Microbit v2 implements digital level

use crate::level::bubble::Bubble;

const CENTER_ROW_OFFSET: i8 = 2;
const CENTER_COL_OFFSET: i8 = 2;
const BUBBLE_MAX_ROW: i8 = 4;
const BUBBLE_MAX_COL: i8 = 4;
// Provide scaling values to make display output provide -500 mG to 500 mG
// reading range, and alternately -50 mG to 50 mG range:
const LEVEL_SCALE_COURSE: f32 = 6.0;
const LEVEL_SCALE_FINE: f32 = 60.0;

// rustc --explain E0616
mod bubble {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Bubble {
        pub row: i8,
        pub col: i8,
    }
}

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
    upside_down: bool
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
            upside_down: false,
        }
    }

    // Note this is a development time diagnostic to observe row and column
    // disply element values:
    pub(crate) fn bubble_x_y(&self) -> Bubble {
        self.bubble
    }

    pub(crate) fn note_upside_down(&mut self, z_accel: f32) -> bool {
        if z_accel < 0.0 {
            self.upside_down = true;
        } else {
            self.upside_down = false;
        }
        self.upside_down
    }

    pub(crate) fn current_render(
        &self,
    ) -> [[u8; 5]; 5] {
        self.display_data.led_array
    }

    pub(crate) fn pixel_on(&mut self, roll: f32, pitch: f32) -> [[u8; 5]; 5] {
        // Clear the grid-wise data rendered on Microbit LED display matrix
        self.display_data.clear();

        // Select scaling value to apply to x-axis and y-axis tilt readings
        // let mut scaling: f32 = LEVEL_SCALE_COURSE;
        let scaling: f32;
        match self.sense_mode {
            SenseResolution::Course => {scaling = LEVEL_SCALE_COURSE},
            SenseResolution::Fine => {scaling = LEVEL_SCALE_FINE},
        }

        // Update the "bubble" represented by an LED with row and column attributes
        let mut new_row = (roll * scaling) as i8;
        let mut new_col = (pitch * scaling) as i8;

        // Apply offsets which center the "bubble" when board is level
        new_row += CENTER_ROW_OFFSET;
        new_col += CENTER_COL_OFFSET;

        // Clip tilt values to the edge of the LED display matrix
        match new_row {
            i8::MIN..=0 => new_row = 0,
            BUBBLE_MAX_ROW..=i8::MAX => new_row = BUBBLE_MAX_ROW,
            _ => { }
        }
        match new_col {
            i8::MIN..=0 => new_col = 0,
            BUBBLE_MAX_COL..=i8::MAX => new_col = BUBBLE_MAX_COL,
            _ => { }
        }
        self.bubble.row = new_row;
        self.bubble.col = new_col;

        self.display_data.pixel_on(new_row, new_col);
        self.display_data.led_array
    }

    // Wrapper to function to update display by clearing and turning on one LED:
    pub(crate) fn update_display(&mut self, roll: f32, pitch: f32) -> [[u8; 5]; 5] {
        if self.upside_down == false {
            self.pixel_on(roll, pitch);
        } else {
            self.display_data.clear();
        }
        self.display_data.led_array
    }

    fn handle_button_a(&mut self) {
        self.sense_mode = SenseResolution::Course;
    }

    fn handle_button_b(&mut self) {
        self.sense_mode = SenseResolution::Fine;
    }

    pub(crate) fn handle_buttons(&mut self, binput: ButtonPress) {
        match binput {
            ButtonPress::ButtonA => { self.handle_button_a() },
            ButtonPress::ButtonB => { self.handle_button_b() },
            ButtonPress::None => { }
        }
    }
}
