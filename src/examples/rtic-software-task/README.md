# Blink on board LED using RTIC Software Task

This is the variation of the `blinky` example using the [RTIC framework](https://rtic.rs/).

![Blinking on board led on Pico board](/src/examples/rtic-software-task/rtic_software_task.gif)

It is based on `pico` example from [rtic-rc/rtic-examples](https://github.com/rtic-rs/rtic-examples/tree/master/rtic_v1/rp-pico_local_initilzd_resources) repo.

## Usage

When your Raspberry Pi Pico is in USB mass storage device mode simply run:

```sh
cargo run --release --example --rtic-software-task
```

## Additional hardware required

None.

> Of course you will need the Pico board and micro usb cable though.
