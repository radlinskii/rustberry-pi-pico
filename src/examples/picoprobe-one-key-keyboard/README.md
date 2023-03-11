# Picoprobe One Key Keyboard

In this example when the key is pressed the letter `a` is send to the host.
Example is created using [Keyberon](https://github.com/TeXitoi/keyberon) crate, with just one layer, containing only one letter -- `a`. Keyboard matrix contains single column (GPIO13, input), and single row (GPIO14, output).
It has enabled logging information into the console of the host using [defmt](https://github.com/knurling-rs/defmt).

![sending key press event to host and displaying logs in the console on the host](/src/examples/picoprobe-one-key-keyboard/picoprobe_one_key_keyboard.gif)

## Usage

To get the logs in the console it should be run using the `probe-run` runner. See `.cargo/config.toml` to configure it.

```sh
cargo run --release --example --picoprobe-one-key-keyboard
```

## Additional hardware required

You need additional `Raspberry Pi Pico` to setup the `picoprobe` which will enable you debugging the `Pico` board.
For setup instructions see I recommend reading https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry and https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html#debugging-using-another-raspberry-pi-pico
