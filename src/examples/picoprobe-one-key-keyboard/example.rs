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
    peripherals = true,
    dispatchers = [SW0_IRQ]
)]
mod app {
    use defmt::*;
    use defmt_rtt as _;
    use rp_pico::{
        hal::{clocks, gpio::DynPin, sio::Sio, usb::UsbBus, watchdog::Watchdog},
        XOSC_CRYSTAL_FREQ,
    };

    use keyberon::debounce::Debouncer;
    use keyberon::key_code;
    use keyberon::layout::{self, Layout};
    use keyberon::matrix::Matrix;
    use usb_device::class_prelude::*;
    use usb_device::device::UsbDeviceState;

    static mut USB_BUS: Option<usb_device::bus::UsbBusAllocator<rp2040_hal::usb::UsbBus>> = None;

    pub static LAYERS: keyberon::layout::Layers<1, 1, 1> = keyberon::layout::layout! {
        {
            [
                A
            ]
        }
    };

    #[shared]
    struct Shared {
        usb_dev: usb_device::device::UsbDevice<'static, rp2040_hal::usb::UsbBus>,
        usb_class: keyberon::hid::HidClass<
            'static,
            rp2040_hal::usb::UsbBus,
            keyberon::keyboard::Keyboard<()>,
        >,
        layout: Layout<1, 1, 1>,
    }

    #[local]
    struct Local {
        matrix: Matrix<DynPin, DynPin, 1, 1>,
        debouncer: Debouncer<[[bool; 1]; 1]>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        info!("Init");

        // clocks
        let mut resets = ctx.device.RESETS;
        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut resets,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        // led
        let sio = Sio::new(ctx.device.SIO);
        let pins = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut resets,
        );

        let usb_bus = UsbBusAllocator::new(UsbBus::new(
            ctx.device.USBCTRL_REGS,
            ctx.device.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut resets,
        ));
        unsafe {
            USB_BUS = Some(usb_bus);
        }
        let usb_class = keyberon::new_class(unsafe { USB_BUS.as_ref().unwrap() }, ());
        let usb_dev = keyberon::new_device(unsafe { USB_BUS.as_ref().unwrap() });

        let matrix: Matrix<DynPin, DynPin, 1, 1> = Matrix::new(
            [pins.gpio13.into_pull_up_input().into()],
            [pins.gpio14.into_push_pull_output().into()],
        )
        .unwrap();

        let layout = Layout::new(&LAYERS);
        let debouncer = Debouncer::new([[false; 1]; 1], [[false; 1]; 1], 20);

        (
            Shared {
                usb_dev,
                usb_class,
                layout,
            },
            Local { matrix, debouncer },
            init::Monotonics(),
        )
    }

    #[task(binds = USBCTRL_IRQ, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_rx(c: usb_rx::Context) {
        let mut usb_d = c.shared.usb_dev;
        let mut usb_c = c.shared.usb_class;
        usb_d.lock(|d| {
            usb_c.lock(|c| {
                if d.poll(&mut [c]) {
                    c.poll();
                }
            })
        });
    }

    #[task(priority = 2, shared = [usb_dev, usb_class, layout])]
    fn handle_event(mut c: handle_event::Context, event: Option<layout::Event>) {
        match event {
            None => (),
            Some(e) => {
                c.shared.layout.lock(|l| l.event(e));
                return;
            }
        };

        let report: key_code::KbHidReport = c.shared.layout.lock(|l| l.keycodes().collect());
        if !c
            .shared
            .usb_class
            .lock(|k| k.device_mut().set_keyboard_report(report.clone()))
        {
            return;
        }
        if c.shared.usb_dev.lock(|d| d.state()) != UsbDeviceState::Configured {
            return;
        }
        while let Ok(0) = c.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
    }

    #[task(
        binds = TIMER_IRQ_0,
        priority = 1,
        local = [matrix, debouncer],
    )]
    fn scan_timer_irq(c: scan_timer_irq::Context) {
        let keys_pressed = c.local.matrix.get().unwrap();
        let deb_events = c.local.debouncer.events(keys_pressed);

        for event in deb_events {
            handle_event::spawn(Some(event)).unwrap();
        }
        handle_event::spawn(None).unwrap();
    }
}
