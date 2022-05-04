//! # Badger2040 Blinky Example
//!
//! Blinks the activity LED on a badger2040 board, using an RP2040 Timer in Count-down mode.
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Bring in all the rest of our dependencies from the BSP
use pimoroni_badger2040::prelude::*;

#[entry]
fn main() -> ! {
    let board = bsp::Board::take().unwrap();

    let mut count_down = board.timer.count_down();

    let mut led_pin = board.pins.led.into_push_pull_output();

    // Blink the LED at 1 Hz
    loop {
        // LED on, and wait for 500ms
        led_pin.set_high().unwrap();
        count_down.start(500.milliseconds());
        let _ = nb::block!(count_down.wait());

        // LED off, and wait for 500ms
        led_pin.set_low().unwrap();
        count_down.start(500.milliseconds());
        let _ = nb::block!(count_down.wait());
    }
}
