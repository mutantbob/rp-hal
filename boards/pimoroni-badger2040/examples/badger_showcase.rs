//! Showcase the functionality of the Pimoroni badger2040
//!
#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Bring in all the rest of our dependencies from the BSP
use embedded_graphics::{
    mono_font::{ascii::*, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use embedded_hal::{
    digital::v2::{OutputPin, ToggleableOutputPin},
    timer::CountDown,
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use embedded_time::{duration::*, fixed_point::FixedPoint, rate::units::Extensions};
use pimoroni_badger2040::{
    entry, hal, hal::clocks::Clock, hal::pac, hal::pac::interrupt, hal::spi::Spi, hal::Timer,
};

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        pimoroni_badger2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = pimoroni_badger2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut count_down = timer.count_down();
    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();
    // Get all the basic peripherals, and init clocks/timers
    // Enable 3.3V power or you won't see anything
    let mut power = pins.p3v3_en.into_push_pull_output();
    let _ = power.set_high();

    // Buttons on this chip connect the pin to signal high, so we need to pull them down.
    // Since pin-mode is defined in button struct, we can call into_mode() instead of stating
    // that again - this is equivalent to calling pins.pin_name.into_pull_down_input()
    let buttons = pimoroni_badger2040::Buttons {
        a: pins.sw_a.into_mode(),
        b: pins.sw_b.into_mode(),
        c: pins.sw_c.into_mode(),
        up: pins.sw_up.into_mode(),
        down: pins.sw_down.into_mode(),
        usr: pins.user_sw.into_mode(),
    };

    buttons.enable_interrupt_on_press();

    // Set up the pins for the e-ink display
    let _spi_sclk = pins.sclk.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.mosi.into_mode::<hal::gpio::FunctionSpi>();
    let dc = pins.inky_dc.into_push_pull_output();
    let cs = pins.inky_cs_gpio.into_push_pull_output();
    let busy = pins.inky_busy.into_pull_up_input();
    let reset = pins.inky_res.into_push_pull_output();

    // Create an SPI driver instance for the SPI0 device
    let spi = Spi::<_, _, 8>::new(pac.SPI0);
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        32_000_000u32.Hz(),
        &embedded_hal::spi::MODE_0,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let mut display = uc8151::Uc8151::new(spi, cs, dc, busy, reset);

    // Reset display
    display.reset(&mut delay);
    // Wait for the screen to finish reset
    while display.is_busy() {}

    // Initialise display. We're using the default LUT speed setting here
    let _ = display.setup(&mut delay, uc8151::LUT::Internal);

    let text = "Greetings\nfrom Rust!";
    // Note we're setting the Text color to `Off`.
    // The uc8151 driver is set up to treat Off as Black as that is what the bitmap libraries expect
    let character_style = MonoTextStyle::new(&FONT_9X18_BOLD, BinaryColor::Off);
    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        .paragraph_spacing(6)
        .build();

    // Bounding box for our text. Fill it with the opposite color so we can read the text.
    let bounds = Rectangle::new(Point::new(157, 50), Size::new(uc8151::WIDTH - 157, 0));
    bounds
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    // Create the text box and apply styling options.
    let text_box = TextBox::with_textbox_style(text, bounds, character_style, textbox_style);
    // Draw the text box.
    text_box.draw(&mut display).unwrap();

    let _ = display.update();
    loop {
        let button_state = pimoroni_badger2040::sample_buttons_raw(&buttons);
        if button_state.up() {
            count_down.start(100_u32.milliseconds());
        } else if button_state.down() {
            count_down.start(1000_u32.milliseconds());
        } else {
            count_down.start(500_u32.milliseconds());
        }
        led_pin.toggle().unwrap();
        let _ = nb::block!(count_down.wait());
    }
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn IO_IRQ_BANK0() {
    // // The `#[interrupt]` attribute covertly converts this to `&'static mut Option<LedAndButton>`
    // static mut LED_AND_BUTTON: Option<LedAndButton> = None;

    // // This is one-time lazy initialisation. We steal the variables given to us
    // // via `GLOBAL_PINS`.
    // if LED_AND_BUTTON.is_none() {
    //     cortex_m::interrupt::free(|cs| {
    //         *LED_AND_BUTTON = GLOBAL_PINS.borrow(cs).take();
    //     });
    // }
}

// End of file
