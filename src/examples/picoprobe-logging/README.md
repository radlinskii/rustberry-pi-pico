# Picoprobe logging

This is the variation of the `blinky` example using the [RTIC framework](https://rtic.rs/), which has enabled logging information into the console of the host using [defmt](https://github.com/knurling-rs/defmt).

![Blinking on board led on Pico board and displaying logs in the console on the host](/src/examples/picoprobe-logging/picoprobe_logging.gif)

## Usage

To get the logs in the console it should be run using the `probe-run` runner. See `.cargo/config.toml` to configure it.

```sh
cargo run --release --example --picoprobe-logging
```

## Additional hardware required

You need additional `Raspberry Pi Pico` to setup the `picoprobe` which will enable you debugging the `Pico` board.
For setup instructions see I recommend reading https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry and https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html#debugging-using-another-raspberry-pi-pico
