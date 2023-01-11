# Blink outer LED using RTIC

This is almost the same as the `blinky` example, but we're blinking outer LED connected to GPIO 15 and we're using the [RTIC framework](https://rtic.rs/) to achieve that.

![Blinking outer LED connected to Pico board on breadboard](/src/examples/blink-outer-led/blink_outer_led.gif)

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --rtic-blink-outer-led
```

## Additional hardware required

-   breadboard
-   couple of male-to-male jumper cables
-   LED
-   330 Ohms resistor
