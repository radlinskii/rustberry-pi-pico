//! One key keyboard example using keyberon crate.
//! Based on https://github.com/camrbuss/pinci implementation.
//! Currently not working :(

#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true, dispatchers = [PIO0_IRQ_0])]
mod app {
    use cortex_m::prelude::_embedded_hal_watchdog_Watchdog;
    use cortex_m::prelude::_embedded_hal_watchdog_WatchdogEnable;
    use embedded_hal::digital::v2::{InputPin, OutputPin};
    use keyberon::action::{k, l, Action, HoldTapAction, HoldTapConfig};
    use keyberon::chording::{ChordDef, Chording};
    use keyberon::debounce::Debouncer;
    use keyberon::key_code::{self, KeyCode::*};
    use keyberon::layout::{self, Layout};
    use keyberon::matrix::Matrix;
    use rp_pico::{
        hal::{
            self, clocks::init_clocks_and_plls, gpio::DynPin, sio::Sio, timer::Alarm, usb::UsbBus,
            watchdog::Watchdog,
        },
        XOSC_CRYSTAL_FREQ,
    };
    use usb_device::class_prelude::*;
    use usb_device::device::UsbDeviceState;

    use fugit::MicrosDurationU32;

    const SCAN_TIME_US: MicrosDurationU32 = MicrosDurationU32::micros(1000);

    static mut USB_BUS: Option<usb_device::bus::UsbBusAllocator<rp2040_hal::usb::UsbBus>> = None;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum CustomActions {
        Uf2,
        Reset,
    }

    const CHORDS: [ChordDef; 0] = [];

    #[rustfmt::skip]
    pub static LAYERS: keyberon::layout::Layers<1, 1, 1, CustomActions> = keyberon::layout::layout! {
    {[ // 0
        A  ]}
    };

    #[shared]
    struct Shared {
        usb_dev: usb_device::device::UsbDevice<'static, rp2040_hal::usb::UsbBus>,
        usb_class: keyberon::hid::HidClass<
            'static,
            rp2040_hal::usb::UsbBus,
            keyberon::keyboard::Keyboard<()>,
        >,
        uart: rp2040_hal::pac::UART0,
        layout: Layout<1, 1, 1, CustomActions>,
    }

    #[local]
    struct Local {
        watchdog: hal::watchdog::Watchdog,
        chording: Chording<0>,
        matrix: Matrix<DynPin, DynPin, 1, 1>,
        debouncer: Debouncer<[[bool; 1]; 1]>,
        alarm: hal::timer::Alarm0,
        transform: fn(layout::Event) -> layout::Event,
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

        let gpio13 = pins.gpio13;
        let gpio14 = pins.gpio14;

        let mut led = pins.gpio25.into_push_pull_output();
        // GPIO1 is high for the right hand side
        let side = pins.gpio1.into_floating_input();
        // delay for power on
        for _ in 0..1000 {
            cortex_m::asm::nop();
        }

        // Use a transform to get correct layout from right and left side
        let _is_right = side.is_high().unwrap();
        let transform: fn(layout::Event) -> layout::Event = |e| {
            e.transform(|i: u8, j: u8| -> (u8, u8) {
                let x = ((j / 5) * 10) + 4 - (j % 5);
                (i, x)
            })
        };

        // Enable UART0
        resets.reset.modify(|_, w| w.uart0().clear_bit());
        while resets.reset_done.read().uart0().bit_is_clear() {}
        let uart = c.device.UART0;
        uart.uartibrd.write(|w| unsafe { w.bits(0b0100_0011) });
        uart.uartfbrd.write(|w| unsafe { w.bits(0b0011_0100) });
        uart.uartlcr_h.write(|w| unsafe { w.bits(0b0110_0000) });
        uart.uartcr.write(|w| unsafe { w.bits(0b11_0000_0001) });
        uart.uartimsc.write(|w| w.rxim().set_bit());

        // led.set_high().unwrap();
        let matrix: Matrix<DynPin, DynPin, 1, 1> = Matrix::new(
            [gpio13.into_pull_up_input().into()],
            [gpio14.into_push_pull_output().into()],
        )
        .unwrap();

        let layout = Layout::new(&LAYERS);
        let debouncer = Debouncer::new([[false; 1]; 1], [[false; 1]; 1], 20);

        let chording = Chording::new(&CHORDS);

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

        led.set_high().unwrap();

        (
            Shared {
                usb_dev,
                usb_class,
                uart,
                layout,
            },
            Local {
                alarm,
                chording,
                watchdog,
                matrix,
                debouncer,
                transform,
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

    #[task(priority = 2, capacity = 8, shared = [usb_dev, usb_class, layout])]
    fn handle_event(mut c: handle_event::Context, event: Option<layout::Event>) {
        match event {
            // TODO: Support Uf2 for the side not performing USB HID
            // The right side only passes None here and buffers the keys
            // for USB to send out when polled by the host
            None => match c.shared.layout.lock(|l| l.tick()) {
                layout::CustomEvent::Press(event) => match event {
                    CustomActions::Uf2 => {
                        hal::rom_data::reset_to_usb_boot(0, 0);
                    }
                    CustomActions::Reset => {
                        cortex_m::peripheral::SCB::sys_reset();
                    }
                },
                _ => (),
            },
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
        shared = [uart],
        local = [matrix, debouncer, chording, watchdog, alarm, transform],
    )]
    fn scan_timer_irq(mut c: scan_timer_irq::Context) {
        let alarm = c.local.alarm;
        alarm.clear_interrupt();
        let _ = alarm.schedule(SCAN_TIME_US);

        c.local.watchdog.feed();
        let keys_pressed = c.local.matrix.get().unwrap();
        let deb_events = c
            .local
            .debouncer
            .events(keys_pressed)
            .map(c.local.transform);
        // TODO: right now chords cannot only be exclusively on one side
        let events = c.local.chording.tick(deb_events.collect()).into_iter();

        // coordinate and press/release is encoded in a single byte
        // the first 6 bits are the coordinate and therefore cannot go past 63
        // The last bit is to signify if it is the last byte to be sent, but
        // this is not currently used as serial rx is the highest priority
        // end? press=1/release=0 key_number
        //   7         6            543210
        let mut es: [Option<layout::Event>; 16] = [None; 16];
        for (i, e) in events.enumerate() {
            es[i] = Some(e);
        }
        let stop_index = es.iter().position(|&v| v == None).unwrap();
        for i in 0..(stop_index + 1) {
            let mut byte: u8;
            if let Some(ev) = es[i] {
                if ev.coord().1 <= 0b0011_1111 {
                    byte = ev.coord().1;
                } else {
                    byte = 0b0011_1111;
                }
                byte |= (ev.is_press() as u8) << 6;
                if i == stop_index + 1 {
                    byte |= 0b1000_0000;
                }
                // Watchdog will catch any possibility for an infinite loop
                while c.shared.uart.lock(|u| u.uartfr.read().txff().bit_is_set()) {}
                c.shared
                    .uart
                    .lock(|u| u.uartdr.write(|w| unsafe { w.data().bits(byte) }));
            }
        }
    }

    // What is this for?
    #[task(binds = UART0_IRQ, priority = 4, shared = [uart])]
    fn rx(mut c: rx::Context) {
        // RX FIFO is disabled so we just check that the byte received is valid
        // and then we read it. If a bad byte is received, it is possible that the
        // receiving side will never read. TODO: fix this
        if c.shared.uart.lock(|u| {
            u.uartmis.read().rxmis().bit_is_set()
                && u.uartfr.read().rxfe().bit_is_clear()
                && u.uartdr.read().oe().bit_is_clear()
                && u.uartdr.read().be().bit_is_clear()
                && u.uartdr.read().pe().bit_is_clear()
                && u.uartdr.read().fe().bit_is_clear()
        }) {
            let d: u8 = c.shared.uart.lock(|u| u.uartdr.read().data().bits());
            if (d & 0b01000000) > 0 {
                handle_event::spawn(Some(layout::Event::Press(0, d & 0b0011_1111))).unwrap();
            } else {
                handle_event::spawn(Some(layout::Event::Release(0, d & 0b0011_1111))).unwrap();
            }
        }
    }
}
