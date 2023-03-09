//! # Blink On-board LED using RTIC Software Task with debug messages.
//!
//! Blinks the LED on a Pico board.
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for
//! the on-board LED. It also uses `defmt` to send log messages to the host.
//! Should be run with `probe-run` runner, see `.cargo/config.toml` file.

#![no_std]
#![no_main]

use panic_probe as _;

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [SW0_IRQ]
)]
mod app {
    use defmt::*;
    use defmt_rtt as _;
    use rp2040_monotonic::{fugit::Duration, Rp2040Monotonic};
    use rp_pico::hal::{
        clocks, gpio, gpio::pin::bank0::Gpio25, gpio::pin::PushPullOutput, sio::Sio,
        watchdog::Watchdog,
    };
    use rp_pico::XOSC_CRYSTAL_FREQ;

    use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

    const MONO_NUM: u32 = 1;
    const MONO_DENOM: u32 = 1000000;
    const ONE_SEC_TICKS: u64 = 1000000;
    const ONE_SECOND_DURATION: Duration<u64, MONO_NUM, MONO_DENOM> =
        Duration::<u64, MONO_NUM, MONO_DENOM>::from_ticks(ONE_SEC_TICKS);

    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Rp2040Mono = Rp2040Monotonic;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::Pin<Gpio25, PushPullOutput>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        info!("Init");

        // clocks
        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let _clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        // led
        let sio = Sio::new(ctx.device.SIO);
        let gpioa = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );
        let mut led = gpioa.led.into_push_pull_output();
        led.set_low().unwrap();

        // monotonics
        let mono = Rp2040Mono::new(ctx.device.TIMER);

        // blink
        blink::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led, is_on: bool = false])]
    fn blink(ctx: blink::Context) {
        ctx.local.led.toggle().unwrap();
        *ctx.local.is_on = !*ctx.local.is_on;
        info!("{}", if *ctx.local.is_on { "ON!" } else { "OFF!" });
        blink::spawn_after(ONE_SECOND_DURATION).unwrap();
    }
}
