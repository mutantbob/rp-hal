//! HAL for the RP2040 microcontroller
//!
//! This is an implementation of the [`embedded-hal`] traits for the RP2040 microcontroller
//! NOTE This HAL is still under active development. This API will remain volatile until 1.0.0

#![warn(missing_docs)]
#![no_std]
#![cfg_attr(feature = "embassy-traits", feature(generic_associated_types))]
#![cfg_attr(feature = "embassy-traits", feature(type_alias_impl_trait))]

extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
pub use paste;

pub extern crate rp2040_pac as pac;

pub mod adc;
pub(crate) mod atomic_register_access;
pub mod clocks;
mod critical_section_impl;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod multicore;
pub mod pio;
pub mod pll;
pub mod prelude;
pub mod pwm;
pub mod resets;
pub mod rom_data;
pub mod rosc;
pub mod rtc;
pub mod sio;
pub mod spi;
pub mod ssi;
pub mod timer;
pub mod typelevel;
pub mod uart;
pub mod usb;
pub mod watchdog;
pub mod xosc;

// Provide access to common datastructures to avoid repeating ourselves
pub use adc::Adc;
pub use clocks::Clock;
pub use i2c::I2C;
pub use sio::Sio;
pub use spi::Spi;
pub use timer::Timer;
pub use watchdog::Watchdog;

/// Macro to create a mutable reference to a statically allocated value
///
/// This macro returns a value with type `Option<&'static mut $ty>`. `Some($expr)` will be returned
/// the first time the macro is executed; further calls will return `None`. To avoid `unwrap`ping a
/// `None` variant the caller must ensure that the macro is called from a function that's executed
/// at most once in the whole lifetime of the program.
///
/// # Example
///
/// ``` no_run
/// use rp2040_hal;
///
/// fn main() {
///     // OK if `main` is executed only once
///     let x: &'static mut bool = rp2040_hal::singleton!(: bool = false).unwrap();
///
///     let y = alias();
///     // BAD this second call to `alias` will definitively `panic!`
///     let y_alias = alias();
/// }
///
/// fn alias() -> &'static mut bool {
///     singleton!(: bool = false).unwrap()
/// }
/// ```
#[macro_export]
macro_rules! singleton {
    (: $ty:ty = $expr:expr) => {
        critical_section::with(|_| {
            static mut VAR: Option<$ty> = None;

            #[allow(unsafe_code)]
            let used = unsafe { VAR.is_some() };
            if used {
                None
            } else {
                let expr = $expr;

                #[allow(unsafe_code)]
                unsafe {
                    VAR = Some(expr)
                }

                #[allow(unsafe_code)]
                unsafe {
                    VAR.as_mut()
                }
            }
        })
    };
}
