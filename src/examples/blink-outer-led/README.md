# Blink outer LED

This is almost the same as the `blinky` example, but we're blinking outer LED connected to GPIO 15.

![Blinking outer LED connected to Pico board on breadboard](/src/examples/blink-outer-led/blink_outer_led.gif)

It is based on the official getting started tutorial from [Raspberry Pi Pico website](https://projects.raspberrypi.org/en/projects/getting-started-with-the-pico/6)

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --blink-outer-led
```

## Additional hardware required

-   breadboard
-   couple of male-to-male jumper cables
-   LED
-   330 Ohms resistor
