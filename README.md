# Rustberry Pi Pico 

This repo contains various programs I implemented in my new favourite programming language: [Rust](https://www.rust-lang.org)<br/> and run on my new favourite ~~toy~~ runtime environment: [Raspberry Pi Pico](https://www.raspberrypi.com/products/raspberry-pi-pico/).

## Description

The ultimate goal for this repo is to implement a keyboard firmware in **Rust** and run it on *Raspberry Pi Pico*,<br/>
but as I need to learn working with **embedded rust** first, I decided to put any projects I run on my **Pico** as a separate *binary crate* in `src/binary`. Those programs will probably include experimenting with different features of the **Pico**.

## Getting Started

### Dependencies

First of all, you need a [Raspberry Pi Pico](https://www.raspberrypi.com/products/raspberry-pi-pico/) board, along with soldering tools and whatever addtional hardware you would like to work with the **Pico** board.

You need to have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
Once you do, you need to add correct build target to your **Rust** environment: 
```sh
rustup target add thumbv6m-none-eabi
```
Then, you will need a tool that converts **Rust** compilation output into a binary executable by the **Pico** board:
```sh
cargo install elf2uf2-rs
```

### Installation

Clone this repo with:
```sh
git clone https://github.com/radlinskii/rustberry-pi-pico.git
```

Then to try to compile one of the programs from `src/bin` directory, e.g.:
```sh
cargo build --release --bin blinky
```

If it compiles you are good to go.

## Usage

To install chosen firmware to your **Pico** first you need to connect the board to your computer with USB cable while pressing the `BOOTSEL` button on the board.<br/>
This will put your Raspberry Pi Pico into USB mass storage device mode so will be ready to receive new firmware.

Next you can install new firmware on your board to do that simply run one of the programs from `src/bin` folder with `cargo run`, e.g.:
```sh
cargo run --release --bin --blinky
```

That's it, in case of the example above you should see that the LED that is preinstalled on your board starts blinking.

## License

This project is licensed under the MIT License - see the `license` file for details.

## Acknowledgments

Binaries I create, at least at the very beginning, will be inspired by `examples` directory from awesome [rp-hal-boards](https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico) repo.
