# Blink on board LED using RTIC

This is the variation of the `blinky` example using the [RTIC framework](https://rtic.rs/).

![Blinking on board led on Pico board](/src/examples/rtic-blink-on-board-led/rtic_blink_on_board_led.gif)

It is based on `Pico RTIC` example from the awesome [rp-pico](https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_rtic.rs) crate.

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --rtic-blink-on-board-led
```

## Additional hardware required

None.

> Of course you will need the Pico board and micro usb cable though.
