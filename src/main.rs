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
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};
use microbit::hal::uarte::{self, Baudrate, Parity};
use microbit::{
   Board,
   hal::{
       timer::Timer
   },  
   display::blocking::Display,
};
use serial_setup::UartePort;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

// https://docs.rs/micromath/latest/micromath/index.html
#[allow(unused_imports)]
use micromath::F32Ext;

// Local-to-project crates
use crate::level::{Level};
use crate::buttons::{init_buttons};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

const DISPLAY_HOLD_MS: u32 = 200;

#[entry]
fn main() -> ! {
    rtt_init_print!();

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

    rprintln!("- RTT enabled for bubble level demonstration");
    let mut _res: Result<(), uarte::Error>;
    let _res = write!(serial, "The accelerometer chip's id is: {:#b}\n\r", acc[0]);
    let _res = write!(serial, "The magnetometer chip's id is: {:#b}\n\r", mag[0]);
    match _res {
        Ok(_) => rprintln!("write! call good"),
        Err(_) => rprintln!("write! call failed"),
    };

    // From Discover MB2 Book chapter 12, example/show-accel.rs:
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();

    init_buttons(board.GPIOTE, board.buttons);

    loop {
        let (_x, _y, _z) = sensor.acceleration().unwrap().xyz_mg();

        if sensor.accel_status().unwrap().xyz_new_data() {
            let (_x, _y, _z) = sensor.acceleration().unwrap().xyz_mg();
        };

        let xf: f32 = _x as f32;
        let yf: f32 = _y as f32;
        let zf: f32 = _z as f32;

        // Ref https://forum.arduino.cc/t/sensing-tilt-using-accelerometer-alone/135717/5
        // pitch=atan2(ax,sqrt((ay*ay)+(az*az)))
        // roll=atan2(ay,sqrt((ax*ax)+(az*az)))

        // Notes for pitch and roll equations:
        // let mut pitch = xf.atan2((yf * yf + zf * zf).sqrt()).unwrapped_as();
        // let mut roll = yf.atan2((xf * xf + zf * zf).sqrt()).unwrapped_as();

        // Calculate pitch and roll so that we have a known constant value
        // through scaling when board becomes vertical in X and or Y axes:
        let arg_b = (yf * yf + zf * zf).sqrt();
        let pitch = xf.atan2(arg_b);

        let arg_b = (xf * xf + zf * zf).sqrt();
        let roll = yf.atan2(arg_b);

        // DEV BEGIN ----------------------------------------------------------
        let bubble_1 = level.bubble_x_y();
        let _ = write!(serial, "(2) bubble coordinates row and col: {}, {}\n\r",
        bubble_1.row, bubble_1.col);
        // DEV END ------------------------------------------------------------

        use crate::level::ButtonPress;
        let bp: ButtonPress = buttons::read_buttons(false);
        level.handle_buttons(bp);

        let _image = level.pixel_on(
             1.0 * roll,
            -1.0 * pitch
        );
        let image = level.current_render();

        let _ = write!(serial, "pitch and roll: {} {}\n\r", pitch, roll);
        display.show(&mut timer, image, DISPLAY_HOLD_MS);
    }
}
