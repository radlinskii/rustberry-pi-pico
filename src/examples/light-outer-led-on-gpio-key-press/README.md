# Light outer LED on GPIO key press

In this example when the key is pressed the outer LED will light up.
When the key is released the LED will stop shining as well.
The key switch is connecting GPIO 13 which is an input, with GPIO 14 which is an output.

![Outer led connected to Pico board lightning up when key is pressed](/src/examples/light-outer-led-on-gpio-key-press/light_outer_led_on_gpio_key_press.gif)

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --light-outer-led-on-gpio-key-press
```

## Additional hardware required

-   breadboard
-   couple of male-to-male jumper cables
-   LED
-   330 Ohms resistor
-   keyboard switch with connected (soldered in my case) male jumper cables (female-to-male cables might be a workaround to not solder anything)
