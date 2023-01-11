# USB Serial Communicator

This example creates USB device echoing anything it receives.

It is based on `Pico USB Serial` example from the awesome [rp-pico](https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_usb_serial.rs) crate.

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --usb-serial-communicator
```

## Additional hardware required

None.

> Of course you will need the Pico board and micro usb cable though.

## Connecting to serial port on MacOS

To list available serial ports run:

```sh
ls /dev/tty.*
```

Then, when you see the port you are looking for you can connect to it via `screen` command, e.g.:

```sh
screen /dev/tty.usbmodemTEST1
```
