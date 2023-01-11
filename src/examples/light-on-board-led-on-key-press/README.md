# Light on board LED on key press

In this example when the key is pressed the on-board LED will light up.
When the key is released the LED will stop shining as well.
The key switch is connecting GPIO 13 with GND pin.

![On board led on Pico board lightning up when key is pressed](/src/examples/light-on-board-led-on-key-press/light_on_board_led_on_key_press.gif)

It is based on [rp-pico's GPIO IN/OUT example](https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_gpio_in_out.rs).

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --light-on-board-led-on-key-press
```

## Additional hardware required

-   keyboard switch with connected (soldered in my case) male jumper cables (female-to-male cables might be a workaround to not solder anything)
