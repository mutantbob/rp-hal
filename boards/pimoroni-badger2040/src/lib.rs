#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub use hal::pac;

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

/// Provides access to board peripherals on badger2040
#[allow(non_snake_case)]
#[allow(unused)]
pub struct Board {
    pub pins: Pins,
    /// Core peripheral: Cache and branch predictor maintenance operations
    pub CBP: pac::CBP,

    /// Core peripheral: CPUID
    pub CPUID: pac::CPUID,

    /// Core peripheral: Debug Control Block
    pub DCB: pac::DCB,

    /// Core peripheral: Data Watchpoint and Trace unit
    pub DWT: pac::DWT,

    /// Core peripheral: Flash Patch and Breakpoint unit
    pub FPB: pac::FPB,

    /// Core peripheral: Instrumentation Trace Macrocell
    pub ITM: pac::ITM,

    /// Core peripheral: Memory Protection Unit
    pub MPU: pac::MPU,

    /// Core peripheral: Nested Vector Interrupt Controller
    pub NVIC: pac::NVIC,

    /// Core peripheral: System Control Block
    pub SCB: pac::SCB,

    /// Core peripheral: SysTick Timer
    pub SYST: pac::SYST,

    /// Core peripheral: Trace Port Interface Unit
    pub TPIU: pac::TPIU,

    // PAC peripherals

    // pub ADC: pac::ADC,
    pub BUSCTRL: pac::BUSCTRL,
    // pub CLOCKS: pac::CLOCKS,
    pub DMA: pac::DMA,
    pub I2C0: pac::I2C0,
    pub I2C1: pac::I2C1,
    // pub IO_BANK0: pac::IO_BANK0,
    pub IO_QSPI: pac::IO_QSPI,
    pub PIO0: pac::PIO0,
    pub PIO1: pac::PIO1,
    // pub PLL_SYS: pac::PLL_SYS,
    // pub PLL_USB: pac::PLL_USB,
    pub PPB: pac::PPB,
    pub PSM: pac::PSM,
    pub PWM: pac::PWM,
    pub RESETS: pac::RESETS,
    pub ROSC: pac::ROSC,
    pub RTC: pac::RTC,
    // pub SIO: pac::SIO,
    pub SPI0: pac::SPI0,
    pub SPI1: pac::SPI1,
    pub SYSCFG: pac::SYSCFG,
    pub SYSINFO: pac::SYSINFO,
    pub TBMAN: pac::TBMAN,
    // pub TIMER: pac::TIMER,
    pub UART0: pac::UART0,
    pub UART1: pac::UART1,
    pub USBCTRL_DPRAM: pac::USBCTRL_DPRAM,
    pub USBCTRL_REGS: pac::USBCTRL_REGS,
    pub VREG_AND_CHIP_RESET: pac::VREG_AND_CHIP_RESET,
    // pub WATCHDOG: pac::WATCHDOG,
    pub XIP_CTRL: pac::XIP_CTRL,
    pub XIP_SSI: pac::XIP_SSI,
    // pub XOSC: pac::XOSC,

    // HAL drivers
    pub clocks: rp2040_hal::clocks::ClocksManager,
    pub adc: rp2040_hal::Adc,
    pub timer: rp2040_hal::Timer,
}

use hal::gpio::bank0;
use hal::gpio;
pub struct Buttons {
    pub a: rp2040_hal::gpio::Pin<
        bank0::Gpio12,
        gpio::Input<gpio::Floating>,
    >,
    pub b: rp2040_hal::gpio::Pin<
        bank0::Gpio13,
        gpio::Input<gpio::Floating>,
    >,
    pub c: rp2040_hal::gpio::Pin<
        bank0::Gpio14,
        gpio::Input<gpio::Floating>,
    >,
    pub up: rp2040_hal::gpio::Pin<
        bank0::Gpio15,
        gpio::Input<gpio::Floating>,
    >,
    pub down: rp2040_hal::gpio::Pin<
        bank0::Gpio11,
        gpio::Input<gpio::Floating>,
    >,
}

// TODO: macroify this?
// impl Buttons {
//     pub fn new(board: &Board) -> Buttons{
//         Buttons{
//             a: board.pins.sw_a.into_floating_input(),
//             b: board.pins.sw_b.into_floating_input(),
//             c: board.pins.sw_c.into_floating_input(),
//             up: board.pins.sw_up.into_floating_input(),
//             down: board.pins.sw_down.into_floating_input(),
//         }
//     }
// }

pub mod prelude {
    pub use crate as bsp;
    pub use crate::Pins;
    pub use crate::BOOT2_FIRMWARE as _;
    pub use crate::XOSC_CRYSTAL_FREQ;
    pub use core::iter::once;
    pub use cortex_m_rt::entry;
    pub use embedded_hal::digital::v2::InputPin;
    pub use embedded_hal::digital::v2::OutputPin;
    pub use embedded_hal::timer::CountDown;
    pub use embedded_time::duration::Extensions;
    pub use embedded_time::fixed_point::FixedPoint;
    pub use rp2040_hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        prelude::*,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    };
}
impl Board {
    pub fn take() -> Option<Self> {
        Some(Self::new(
            pac::Peripherals::take()?,
            pac::CorePeripherals::take()?,
        ))
    }

    pub fn new(mut p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {
        use prelude::*;
        let mut watchdog = Watchdog::new(p.WATCHDOG);
        let sio = Sio::new(p.SIO);

        let clocks = init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            p.XOSC,
            p.CLOCKS,
            p.PLL_SYS,
            p.PLL_USB,
            &mut p.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let pins = bsp::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

        let adc = bsp::hal::Adc::new(p.ADC, &mut p.RESETS);

        let timer = Timer::new(p.TIMER, &mut p.RESETS);

        Self {
            pins,

            // Core peripherals
            CBP: cp.CBP,
            CPUID: cp.CPUID,
            DCB: cp.DCB,
            DWT: cp.DWT,
            FPB: cp.FPB,
            ITM: cp.ITM,
            MPU: cp.MPU,
            NVIC: cp.NVIC,
            SCB: cp.SCB,
            SYST: cp.SYST,
            TPIU: cp.TPIU,

            // RP2040 peripherals
            // ADC: pac.ADC,
            BUSCTRL: p.BUSCTRL,
            // CLOCKS: pac.CLOCKS,
            DMA: p.DMA,
            I2C0: p.I2C0,
            I2C1: p.I2C1,
            // IO_BANK0: pac.IO_BANK0,
            IO_QSPI: p.IO_QSPI,
            PIO0: p.PIO0,
            PIO1: p.PIO1,
            // PLL_SYS: pac.PLL_SYS,
            // PLL_USB: pac.PLL_USB,
            PPB: p.PPB,
            PSM: p.PSM,
            PWM: p.PWM,
            RESETS: p.RESETS,
            ROSC: p.ROSC,
            RTC: p.RTC,
            // SIO: pac.SIO,
            SPI0: p.SPI0,
            SPI1: p.SPI1,
            SYSCFG: p.SYSCFG,
            SYSINFO: p.SYSINFO,
            TBMAN: p.TBMAN,
            // TIMER: p.TIMER,
            UART0: p.UART0,
            UART1: p.UART1,
            USBCTRL_DPRAM: p.USBCTRL_DPRAM,
            USBCTRL_REGS: p.USBCTRL_REGS,
            VREG_AND_CHIP_RESET: p.VREG_AND_CHIP_RESET,
            // WATCHDOG: pac.WATCHDOG,
            XIP_CTRL: p.XIP_CTRL,
            XIP_SSI: p.XIP_SSI,
            // XOSC: pac.XOSC,
            clocks,
            adc,
            timer,
        }
    }
}
