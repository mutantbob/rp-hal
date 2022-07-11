#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
use hal::gpio;
use hal::gpio::bank0;
use hal::gpio::Interrupt;
pub use hal::pac;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

hal::bsp_pins!(
    Gpio0 {
        name: gpio0,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio0].
            FunctionUart: UartTx
        }
    },
    Gpio1 {
        name: gpio1,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio1].
            FunctionUart: UartRx
        }
    },
    Gpio3 { name: i2c_int },
    Gpio4 {
        name: gpio4,
        aliases: {
            /// I2C Function alias for pin [crate::Pins::gpio4].
            FunctionI2C: I2cSda
        }
    },
    Gpio5 {
        name: gpio5,
        aliases: {
            /// I2C Function alias for pin [crate::Pins::gpio5].
            FunctionI2C: I2cScl
        }
    },
    Gpio10 { name: p3v3_en },
    Gpio11 { name: sw_down },
    Gpio12 { name: sw_a },
    Gpio13 { name: sw_b },
    Gpio14 { name: sw_c },
    Gpio15 { name: sw_up },
    Gpio16 {
        name: miso,
        aliases: {
            /// SPI Function alias for pin [crate::Pins::gpio16].
            FunctionSpi: Miso
        }
    },
    Gpio17 {
        name: inky_cs_gpio,
        aliases: {
            /// SPI Function alias for pin [crate::Pins::gpio17].
            FunctionSpi: InkyCs
        }
    },
    Gpio18 {
        name: sclk,
        aliases: {
            /// SPI Function alias for pin [crate::Pins::gpio18].
            FunctionSpi: Sclk
        }
    },
    Gpio19 {
        name: mosi,
        aliases: {
            /// SPI Function alias for pin [crate::Pins::gpio19].
            FunctionSpi: Mosi
        }
    },
    Gpio20 { name: inky_dc },
    Gpio21 { name: inky_res },
    Gpio23 { name: user_sw },
    /// GPIO 24 is connected to vbus_detect of the badger2040.
    Gpio24 { name: vbus_detect },
    /// GPIO 25 is connected to activity LED of the badger2040.
    Gpio25 { name: led },
    Gpio26 { name: inky_busy },
    Gpio27 { name: vref_power },
    Gpio28 { name: vref_1v24 },
    /// GPIO 29 is connected to battery monitor of the badger2040
    Gpio29 { name: vbat_sense },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

pub struct Buttons {
    pub a: rp2040_hal::gpio::Pin<bank0::Gpio12, gpio::Input<gpio::PullDown>>,
    pub b: rp2040_hal::gpio::Pin<bank0::Gpio13, gpio::Input<gpio::PullDown>>,
    pub c: rp2040_hal::gpio::Pin<bank0::Gpio14, gpio::Input<gpio::PullDown>>,
    pub up: rp2040_hal::gpio::Pin<bank0::Gpio15, gpio::Input<gpio::PullDown>>,
    pub down: rp2040_hal::gpio::Pin<bank0::Gpio11, gpio::Input<gpio::PullDown>>,
    pub usr: rp2040_hal::gpio::Pin<bank0::Gpio23, gpio::Input<gpio::Floating>>,
}

impl Buttons {
    fn interrupt_change(&self, interrupt: Interrupt, enabled: bool) {
        self.a.set_interrupt_enabled(interrupt, enabled);
        self.b.set_interrupt_enabled(interrupt, enabled);
        self.c.set_interrupt_enabled(interrupt, enabled);
        self.up.set_interrupt_enabled(interrupt, enabled);
        self.down.set_interrupt_enabled(interrupt, enabled);
        // Polarity of user button is reversed, so invert interrupt edge here.
        let interrupt_swap_edge = if interrupt == hal::gpio::Interrupt::EdgeLow {
            hal::gpio::Interrupt::EdgeHigh
        } else {
            hal::gpio::Interrupt::EdgeLow
        };
        self.usr.set_interrupt_enabled(interrupt_swap_edge, enabled);
    }
    /// Enable triggering interrupt on button presses.
    /// This will happen on the 'rising edge' of the input pins
    pub fn enable_interrupt_on_press(&self) {
        self.interrupt_change(hal::gpio::Interrupt::EdgeHigh, true);
    }
    /// Disable triggering interrupt on button presses.
    /// This will happen on the 'rising edge' of the input pins
    pub fn disable_interrupt_on_press(&self) {
        self.interrupt_change(hal::gpio::Interrupt::EdgeHigh, false);
    }
    /// Enable triggering interrupt on button presses.
    /// This will happen on the 'rising edge' of the input pins
    pub fn enable_interrupt_on_release(&self) {
        self.interrupt_change(hal::gpio::Interrupt::EdgeLow, true);
    }
    /// Disable triggering interrupt on button presses.
    /// This will happen on the 'rising edge' of the input pins
    pub fn disable_interrupt_on_release(&self) {
        self.interrupt_change(hal::gpio::Interrupt::EdgeLow, false);
    }
}

pub struct ButtonsRaw {
    pub data: u8,
}

/// Read the state of all buttons, store in a u8 as a bitfield (inside ButtonsRaw).
/// Rely on associated functions for getting button states
pub fn sample_buttons_raw(buttons: &Buttons) -> ButtonsRaw {
    use embedded_hal::digital::v2::InputPin;
    let mut val: u8 = 0;
    // It is safe to unwrap here, as the pin reads are infallible
    // For pins that can fail, you should return a result
    if buttons.a.is_high().unwrap() {
        val += 1 << 0;
    }
    if buttons.b.is_high().unwrap() {
        val += 1 << 1;
    }
    if buttons.c.is_high().unwrap() {
        val += 1 << 2;
    }
    if buttons.up.is_high().unwrap() {
        val += 1 << 3;
    }
    if buttons.down.is_high().unwrap() {
        val += 1 << 4;
    }
    // user_sw logic is inverted compared to other buttons
    if buttons.usr.is_low().unwrap() {
        val += 1 << 5;
    }
    ButtonsRaw { data: val }
}

impl ButtonsRaw {
    pub fn a(&self) -> bool {
        self.data & (1 << 0) != 0
    }
    pub fn b(&self) -> bool {
        self.data & (1 << 1) != 0
    }
    pub fn c(&self) -> bool {
        self.data & (1 << 2) != 0
    }
    pub fn up(&self) -> bool {
        self.data & (1 << 3) != 0
    }
    pub fn down(&self) -> bool {
        self.data & (1 << 4) != 0
    }
    pub fn usr(&self) -> bool {
        self.data & (1 << 5) != 0
    }
}
