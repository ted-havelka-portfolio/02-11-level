# "Level"

**"Bubble Level" demonstration** on Microbit v2 to illustrate simple use of an accelerometer.

**Implementor: Theodore Havelka**

## Demo Specifications (what the app does)

When the board is upside down (
 is positive), the display should be blanked.

Otherwise, a single LED on the display should be lit to show the "level point". Start in "coarse" mode: divide the range from -500 to 500 mG into 5 parts. The on LED should be at the position given by the 
 and 
 coordinates using this scaling. If the LED would be "off the edge" on either axis, clamp it to that axis.

Note that this is a fake bubble level. That is, the lit LED should be toward the higher edge of the board along the 
 and 
 axes; the board can be leveled by lowering the edges to bring the lit LED toward the center. When the on LED is in the center, the 
 and 
 components of acceleration are close to 0 mG, so the MB2 is roughly level.

Pressing the B button (by itself) should put the level in "fine" mode: now the LED scales should go from -50 to 50 mG. Pressing the A button (by itself) should return to "coarse" mode.

The level measurement and display should refresh every 200 ms (5 frames per second).

## How The Demo Works

The level app implements a superloop, which communicates with off-chip sensors and peripherals and updates and LED array as tiny screen .  The screen only ever has one LED turned on, representing a "bubble" as is found in classic glass bulb and water based levels.

Interrupts are employed to capture button presses from a user.  There is no scheduling in this single application, single thread demo.



## Development

New features in this demo include use of I2C and Microbit's accelerometer, the LSM303AGR.  Also new in this work is use of a `no_std` Rust crate for floating point math.  The math crate chosen is `micromath 1.0.0`.

Some early exploration was made into the use of integer math.  The Rust crate `num-integer` provides such a library.  It's documentation says it can be compiled with `no_std` called out.  For the "level" application, however, building this crate without Rust standard proved to be not readily possible.  Division operations which this app uses are not trivial to implement, so use of this library was abandoned.

The level application draws heavily on the earlier 2026 Q1 Game of Life application.  There is a superloop.  There is a module to managebutton presses.  There is a module to implement most of the variables, data structures and logic of the demo, but not all!  The primary loop in `main.rs` is mostly reponsible for gathering sensor and other input device data.  Logic in the `level.rs` file decides how to process this data and further actions to take based on certin input data and events.

I2C implementation was straightfoward, and based on example code from the Rust Embedded Discovery Book v2, chapter 12.https://docs.rust-embedded.org/discovery-mb2/12-i2c/index.html.  The button managing code was brought over from Game of Life with very few changes.  

## Points 
