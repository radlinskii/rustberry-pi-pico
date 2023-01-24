//! # Toggle On-board LED On GPIO Key Press Example
//!
//! Toggles the LED attached to the Pico board the moment the key is pressed.
//!
//! This will toggle the state of the on-board LED
//! when key attached to GPIO13 closes the circuit with GPIO14.

#![no_std]
#![no_main]

use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    let button_input_pin = pins.gpio13.into_pull_up_input();
    pins.gpio14.into_push_pull_output();

    let mut led_on = false;
    let mut prev_button_pressed = false;

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
