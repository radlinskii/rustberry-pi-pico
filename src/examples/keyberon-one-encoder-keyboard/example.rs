//! Single rotary encoder keyboard example using keyberon crate.
//! Based on https://github.com/camrbuss/pinci implementation.

#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true, dispatchers = [PIO0_IRQ_0])]
mod app {
    use cortex_m::prelude::_embedded_hal_watchdog_Watchdog;
    use cortex_m::prelude::_embedded_hal_watchdog_WatchdogEnable;
    use embedded_hal::digital::v2::OutputPin;
    use hal::gpio::bank0::Gpio25;
    use hal::gpio::bank0::Gpio8;
    use hal::gpio::bank0::Gpio9;
    use hal::gpio::PullUpInput;
    use hal::gpio::PushPullOutput;
    use keyberon::debounce::Debouncer;
    use keyberon::key_code;
    use keyberon::layout::{Event, Layout};
    use keyberon::matrix::Matrix;
    use rotary_encoder_hal::{Direction, Rotary};
    use rp_pico::{
        hal::{
            self, clocks::init_clocks_and_plls, gpio::DynPin, gpio::Pin, sio::Sio, timer::Alarm,
            usb::UsbBus, watchdog::Watchdog,
        },
        XOSC_CRYSTAL_FREQ,
    };
    use usb_device::class_prelude::*;
    use usb_device::device::UsbDeviceState;

    use fugit::MicrosDurationU32;

    const SCAN_TIME_US: MicrosDurationU32 = MicrosDurationU32::micros(1000);

    static mut USB_BUS: Option<usb_device::bus::UsbBusAllocator<rp2040_hal::usb::UsbBus>> = None;

    pub static LAYERS: keyberon::layout::Layers<3, 1, 1> = keyberon::layout::layout! {
        {
            [A B C],
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
        layout: Layout<3, 1, 1>,
        encoder: Rotary<Pin<Gpio8, PullUpInput>, Pin<Gpio9, PullUpInput>>,
        led: Pin<Gpio25, PushPullOutput>,
    }

    #[local]
    struct Local {
        watchdog: hal::watchdog::Watchdog,
        matrix: Matrix<DynPin, DynPin, 3, 1>,
        debouncer: Debouncer<[[bool; 3]; 1]>,
        alarm: hal::timer::Alarm0,
        resolution_count: u8,
    }

    #[init]
    fn init(c: init::Context) -> (Shared, Local, init::Monotonics) {
        // Soft-reset does not release the hardware spinlocks
        // Release them now to avoid a deadlock after debug or watchdog reset
        unsafe {
            hal::sio::spinlock_reset();
        }
        let mut resets = c.device.RESETS;
        let mut watchdog = Watchdog::new(c.device.WATCHDOG);
        let clocks = init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            c.device.XOSC,
            c.device.CLOCKS,
            c.device.PLL_SYS,
            c.device.PLL_USB,
            &mut resets,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let sio = Sio::new(c.device.SIO);
        let pins = hal::gpio::Pins::new(
            c.device.IO_BANK0,
            c.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut resets,
        );

        let led = pins.gpio25.into_push_pull_output();

        // delay for power on
        for _ in 0..1000 {
            cortex_m::asm::nop();
        }

        let matrix: Matrix<DynPin, DynPin, 3, 1> = Matrix::new(
            [
                pins.gpio13.into_pull_up_input().into(),
                // those two pins below are not actually connected
                // they are used only to make the matrix type be 3x1
                pins.gpio15.into_pull_up_input().into(),
                pins.gpio16.into_pull_up_input().into(),
            ],
            [pins.gpio14.into_push_pull_output().into()],
        )
        .unwrap();

        let layout = Layout::new(&LAYERS);
        let debouncer = Debouncer::new([[false; 3]; 1], [[false; 3]; 1], 10);

        let encoder_a = pins.gpio8.into_pull_up_input();
        let encoder_b = pins.gpio9.into_pull_up_input();
        let encoder = Rotary::new(encoder_a, encoder_b);

        let mut timer = hal::Timer::new(c.device.TIMER, &mut resets);
        let mut alarm = timer.alarm_0().unwrap();
        let _ = alarm.schedule(SCAN_TIME_US);
        alarm.enable_interrupt();

        let usb_bus = UsbBusAllocator::new(UsbBus::new(
            c.device.USBCTRL_REGS,
            c.device.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut resets,
        ));
        unsafe {
            USB_BUS = Some(usb_bus);
        }
        let usb_class = keyberon::new_class(unsafe { USB_BUS.as_ref().unwrap() }, ());
        let usb_dev = keyberon::new_device(unsafe { USB_BUS.as_ref().unwrap() });

        // Start watchdog and feed it with the lowest priority task at 1000hz
        watchdog.start(MicrosDurationU32::micros(10000));

        let resolution_count: u8 = 0;

        (
            Shared {
                usb_dev,
                usb_class,
                layout,
                encoder,
                led,
            },
            Local {
                alarm,
                watchdog,
                matrix,
                debouncer,
                resolution_count,
            },
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

    #[task(priority = 2, capacity = 8, shared = [usb_dev, usb_class, layout, encoder])]
    fn handle_event(mut c: handle_event::Context, event: Option<Event>) {
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
        shared=[encoder, layout, led],
        local = [matrix, debouncer, watchdog, alarm, resolution_count],
    )]
    fn scan_timer_irq(mut c: scan_timer_irq::Context) {
        let alarm = c.local.alarm;
        alarm.clear_interrupt();
        let _ = alarm.schedule(SCAN_TIME_US);

        c.local.watchdog.feed();
        let keys_pressed = c.local.matrix.get().unwrap();

        let deb_events = c.local.debouncer.events(keys_pressed);

        for event in deb_events {
            handle_event::spawn(Some(event)).unwrap();
        }

        // Read the encoder pins through an update()
        // Then if 4 increments have occurred (one physical click),
        // press and release the "buttons" in the layout
        c.shared.encoder.lock(|e| match e.update().unwrap() {
            Direction::Clockwise => {
                if *c.local.resolution_count == 3 {
                    handle_event::spawn(Some(Event::Press(1, 0))).unwrap();
                    handle_event::spawn(Some(Event::Release(1, 0))).unwrap();

                    c.shared.led.lock(|l| l.set_high().unwrap());

                    *c.local.resolution_count = 0;
                } else {
                    *c.local.resolution_count += 1;
                }
            }
            Direction::CounterClockwise => {
                if *c.local.resolution_count == 3 {
                    handle_event::spawn(Some(Event::Press(2, 0))).unwrap();
                    handle_event::spawn(Some(Event::Release(2, 0))).unwrap();

                    c.shared.led.lock(|l| l.set_low().unwrap());

                    *c.local.resolution_count = 0;
                } else {
                    *c.local.resolution_count += 1;
                }
            }
            Direction::None => {}
        });

        handle_event::spawn(None).unwrap();
    }
}
