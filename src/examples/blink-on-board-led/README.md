# Blink on board LED

This is the most basic example of using Raspberry Pi Pico.

![Blinking on board led on Pico board](/src/examples/blink-on-board-led/blink_on_board_led.gif)

It is based on `blinky` example the from awesome [rp-pico](https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_blinky.rs) crate.

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --blink-on-board-led
```

## Additional hardware required

None.

> Of course you will need the Pico board and micro usb cable though.
