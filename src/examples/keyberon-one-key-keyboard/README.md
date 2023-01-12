# Keyberon One Key Keyboard

In this example when the key is pressed the letter `a` is send to the host.
Example is created using [Keyberon](https://github.com/TeXitoi/keyberon) crate, with just one layer, containing only one letter -- `a`. Keyboard matrix contains single column (GPIO13, input), and single row (GPIO14, output).

![Pressing key attached to Pico is sending letter 'a' to computer via USB](/src/examples/keyberon-one-key-keyboard/keyberon_one_key_keyboard.gif)

Implementation is based on https://github.com/camrbuss/pinci keyboard firmware.

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --keyberon-one-key-keyboard
```

## Additional hardware required

-   keyboard switch with connected (soldered in my case) male jumper cables (female-to-male cables might be a workaround to not solder anything)
