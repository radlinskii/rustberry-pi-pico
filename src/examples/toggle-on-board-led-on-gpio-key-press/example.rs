//! # Toggle On-board LED On GPIO Key Press Example
//!
//! Toggles the LED attached to the Pico board the moment the key is pressed.
//!
//! This will toggle the state of the on-board LED
//! when key attached to GPIO13 closes the circuit with GPIO14.

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;
use rp_pico::hal::prelude::*;

// GPIO traits
use embedded_hal::digital::v2::{InputPin, OutputPin};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    // Set one line of the button to be an input
    let button_input_pin = pins.gpio13.into_pull_up_input();
    // Set second line of the button to be an output
    pins.gpio14.into_push_pull_output();

    let mut led_on = false;
    let mut prev_button_pressed = false;

    // Light the LED when key is pressed
    loop {
        let button_pressed = button_input_pin.is_low().unwrap();
        if button_pressed && !prev_button_pressed {
            if !led_on {
                led_pin.set_high().unwrap();
                led_on = true;
            } else {
                led_pin.set_low().unwrap();
                led_on = false;
            }
        }

        prev_button_pressed = button_pressed;
        delay.delay_ms(100);
    }
}
