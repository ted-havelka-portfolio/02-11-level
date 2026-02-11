#![no_main]
#![no_std]

mod level;
mod buttons;

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use embedded_hal::i2c::I2c;
use embedded_hal::delay::DelayNs;

// Code block to <<EOC
// TODO [ ] learn how following `use` statements can be better factored,
//  hopefully reduced in count and made to be more readable:
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};
use microbit::hal::uarte::{self, Baudrate, Parity};
use microbit::{
   Board,
   hal::{
       timer::Timer
   },  
   display::blocking::Display,
};
// EOC

use serial_setup::UartePort;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
// https://docs.rs/micromath/latest/micromath/index.html
#[allow(unused_imports)]
use micromath::F32Ext;

use crate::level::{Level};
use crate::buttons::{init_buttons};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

const DISPLAY_HOLD_MS: u32 = 200;
const SCALE_FOR_DISPLAY: f32 = 3.0;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // let board = microbit::Board::take().unwrap();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut acc = [0u8];
    let mut mag = [0u8];

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut level = Level::new();
    let mut display = Display::new(board.display_pins);

    timer.delay_ms(10u32);

    // First write the address + register onto the bus, then read the chip's responses
    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc)
        .unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag)
        .unwrap();

    // rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    // rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);
    rprintln!("- RTT enabled for 'level' assignment");
    let mut _res: Result<(), uarte::Error>;
    let _res = write!(serial, "The accelerometer chip's id is: {:#b}\n\r", acc[0]);
    let _res = write!(serial, "The magnetometer chip's id is: {:#b}\n\r", mag[0]);
    match _res {
        Ok(_) => rprintln!("write! call good"),
        Err(_) => rprintln!("write! call failed"),
    };

    // From Discover MB2 Book chapter 12, example/show-accel.rs:
    // borrowed code to <<EOC
    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();
    // EOC

    init_buttons(board.GPIOTE, board.buttons);

    loop {
        let (_x, _y, _z) = sensor.acceleration().unwrap().xyz_mg();

        if sensor.accel_status().unwrap().xyz_new_data() {
            let (_x, _y, _z) = sensor.acceleration().unwrap().xyz_mg();
//            let _res = write!(serial, "Acceleration: x {} y {} z {}\n\r", x, y, z);
//        } else {
//            let _res = write!(serial, "unable to query accelerometer for x, y, z data!\n\r");
        };

        // let mut x_as_f32: f32 = x as f32;
        let xf: f32 = _x as f32;
        let yf: f32 = _y as f32;
        let zf: f32 = _z as f32;

        // Ref https://forum.arduino.cc/t/sensing-tilt-using-accelerometer-alone/135717/5
        // pitch=atan2(ax,sqrt((ay*ay)+(az*az)))
        // roll=atan2(ay,sqrt((ax*ax)+(az*az)))

        // Notes for pitch and roll equations:
        // let mut pitch = xf.atan2((yf * yf + zf * zf).sqrt()).unwrapped_as();
        // let mut roll = yf.atan2((xf * xf + zf * zf).sqrt()).unwrapped_as();

        // Craft an intermediate term pending understanding of how to expression
        // the entire formula in one line of Rust:
        let arg_b = (yf * yf + zf * zf).sqrt();
        let pitch = xf.atan2(arg_b);

        let arg_b = (xf * xf + zf * zf).sqrt();
        let roll = yf.atan2(arg_b);

        // DEV BEGIN ----------------------------------------------------------
        // let _ = write!(serial, "(1) calling set_pixel with {}, {}\n\r",
        //     (1.0 * roll * SCALE_FOR_DISPLAY) as i8,
        //     (-1.0 * pitch * SCALE_FOR_DISPLAY) as i8);

        // let bubble_1 = level.bubble_x_y();
        // let _ = write!(serial, "(2) bubble coordinates row and col: {}, {}\n\r",
        //     bubble_1.row, bubble_1.col);

        let sample_res: u8 = level.sense_mode_c_f();
        let _ = write!(serial, "sensor resolution set to {}\n\r", sample_res);
        // let _ = write!(serial, "'1' for course sampling, '2' means fine\n\r");
        // DEV END ------------------------------------------------------------

        use crate::level::ButtonPress;
        let mut bp: ButtonPress = buttons::read_buttons(false);
        level.handle_buttons(bp);

        let _image = level.pixel_on(
             (1.0 * roll * SCALE_FOR_DISPLAY) as i8,
            (-1.0 * pitch * SCALE_FOR_DISPLAY) as i8
        );
        let image = level.current_render();

        let _ = write!(serial, "pitch and roll: {} {}\n\r", pitch, roll);
        display.show(&mut timer, image, DISPLAY_HOLD_MS);
    }
}
