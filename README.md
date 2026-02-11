# "Level"

**"Bubble Level" demonstration** on Microbit v2 to illustrate simple use of an
accelerometer.

**Implementor: Theodore Havelka**

## Demo Specifications (what the app does)

When a user holds the Microbit board with LED display facing up, a single LED
alights to show how much the board is tilted in the X and Y axes. The scaling of
LED change in the 5x5 array is that each LED away from center, along a row or a
column represents 1/5g, or about 1.96m/s^2 acceleration.

- When the board is upside down (display down) display is blanked.

- When the LED position is calculated "off any edge" of the display, clamp it to
  that axis.

- Qualitatively the LED should move on the display as though it was a bubble in
  a hemispherical water-filled glass bulb.

- To press the B button puts the level in "fine", higher sensitivity mode; the
  LED scales from -50 to 50 mG.

- To press the A button (by itself) returns the demo app to "coarse" reading and
  display mode.

- Acceleration measurement and display refresh every 200 ms (5 frames per
  second).

## How The Demo Is Built

The level app implements a "super loop", a primary loop which runs indefinitely
after some initial start up code. This loop communicates with off-chip sensors
and peripherals and updates the board's LED based display. As described in the
specs, the display only ever has one LED turned on at most, representing a
"bubble" as is found in classic glass bulb and water based levels.

Interrupts are employed to capture button presses from a user. There is no
scheduling in this single application, single thread demo.

The level application draws heavily on the earlier 2026 Q1 Game of Life
application. Source file `main.rs` implements a loop which orchestrates all the
more specific, sometimes lower level tasks. There is a module to manage button
presses. There is a module to implement most of the variables, data structures
and detailed logic of the demo, but not all! The primary loop in `main.rs` is
mostly reponsible for gathering sensor and other input device data. Logic in the
`level.rs` file decides how to process this data and further actions to take
based on certin input data and events.

The not perfectly clear delineation of responsiblities between `main.rs` and
`level.rs` is an area contributor Ted would like to improve.

## Development

With respect to previous work new features in this demo include use of I2C and
Microbit's accelerometer, the LSM303AGR. Also new in this work is use of
`micromath 1.0.0` for floating point math.

Some early exploration was made into the use of integer math. The Rust crate
`num-integer` provides such a library. It's documentation says it can be
compiled with `no_std` called out. For the "level" application, however,
building this crate without Rust standard proved to be not readily possible.
Division operations which this app uses are not trivial to implement, so use of
this library was abandoned.

I2C implementation was straightfoward, and based on example code from the Rust
Embedded Discovery Book v2, chapter
12.https://docs.rust-embedded.org/discovery-mb2/12-i2c/index.html. The button
managing code was brought over from Game of Life with very few changes.

The floating point crate inclusion was the most challenging part of level demo
development. Crate `libm` did not work for dependency issues. A second crate
also failed to be readily integrated with the demo.

In addition to employing floating point operations on accelerometer data, the
level demo app needs to convert between integer and floating point data types
and back again. LSM303AGR readings are integers. Fractional multiplications
require these to be turned into floats temporarily. Final values are used to
select which LED to light in an array whose effective indeces are integer. (Rust
strongly encourages developers to use safer means than integer indeces to class
C-like arrays.) Data conversion needs to occur at the best times, however, to
avoid rounding errors as much as possible. This practical need came into play
during level demo development.

## Things To Explore

Contributor Ted noticed an interesting pattern in the way Rust functions are
sometimes defined in their signatures. In this demo's local crate named `level`
the function `update_display()` has the signature:

```rust
    pub(crate) fn update_display(&mut self, roll: f32, pitch: f32) -> [[u8; 5]; 5] {
                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

but it is called with only the second and successive parameters:

```rust
        let _image = level.update_display(
             1.0 * roll,
            -1.0 * pitch
        );
```

This appears to be a common occurance in Rust, in which function signatures
contain a leading parameter which is not expressed by calling code. This is a
point worth searching to better understand the mechanism at play, and why such
syntax is chosen and makes sense.

Early in the development of the level demonstration some time was spent
researching integer arithmetic libraries for Rust "no_std". While this didn't
pan out contributor Ted did find an interesting chapter on integer division at
[Algorithmica](https://en.algorithmica.org/hpc/arithmetic/division/). This
seeming book section mentions [Henry Warren's book "Hacker's Delight"](<>). It
sounds like this book would have some useful programming techniques to apply in
embedded applications and memory constrained systems.
