# "Level"

**"Bubble Level" demonstration** on Microbit v2 to illustrate simple use of an accelerometer.

**Implementor: Theodore Havelka**

## Demo Specifications (what the app does)

## How The Demo Works

The level app implements a superloop, which communicates with off-chip sensors and peripherals and updates and LED array as tiny screen .  The screen only ever has one LED turned on, representing a "bubble" as is found in classic glass bulb and water based levels.

Interrupts are employed to capture button presses from a user.  There is no scheduling in this single application, single thread demo.



## Development

New features in this demo include use of I2C and Microbit's accelerometer, the LSM303AGR.  Also new in this work is use of a `no_std` Rust crate for floating point math.  The math crate chosen is `micromath 1.0.0`.

Some early exploration was made into the use of integer math.  The Rust crate `num-integer` provides such a library.  It's documentation says it can be compiled with `no_std` called out.  For the "level" application, however, building this crate without Rust standard proved to be not readily possible.  Division operations which this app uses are not trivial to implement, so use of this library was abandoned.

The level application draws heavily on the earlier 2026 Q1 Game of Life application.  There is a superloop.  There is a module to managebutton presses.  There is a module to implement most of the variables, data structures and logic of the demo, but not all!  The primary loop in `main.rs` is mostly reponsible for gathering sensor and other input device data.  Logic in the `level.rs` file decides how to process this data and further actions to take based on certin input data and events.

I2C implementation was straightfoward, and based on example code from the Rust Embedded Discovery Book v2, chapter 12.https://docs.rust-embedded.org/discovery-mb2/12-i2c/index.html.  The button managing code was brought over from Game of Life with very few changes.  

## Points 
