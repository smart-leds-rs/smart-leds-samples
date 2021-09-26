# Raspberry Pi Pico Examples

These examples use the [rp-rs/rp-hal](https://github.com/rp-rs/rp-hal) BSP for the [Raspberry Pi Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/).

Grab the latest [GNU Arm Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).
Packages exist for this toolchain in the Ubuntu and Fedora Linux repositories.
On Fedora Linux, install the necessary packages as follows.

    sudo dnf install arm-none-eabi-gcc-cs arm-none-eabi-newlib

On Ubuntu, use the following command to install the GNU Arm Embedded toolchain.

    sudo apt install gcc-arm-none-eabi libnewlib-arm-none-eabi

Install the Rust toolchain for the Raspberry Pi Pico.

    rustup target add thumbv6m-none-eabi

Install [flip-link](https://github.com/knurling-rs/flip-link)

    cargo install flip-link

## Program

The Raspberry Pi Pico can be programmed directly through its micro USB port or using its SWD pins.

### UF2

[elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs) makes it incredibly easy to flash the Raspberry Pi Pico using its micro USB port.

Install the `elf2uf2-rs` crate.

    cargo install elf2uf2-rs

While holding the `BOOTSEL` button, attach the Raspberry Pi Pico to your computer using the micro USB port.
Flash the board using Cargo's `run` subcommand.

    cargo run --release --example raspberry_pi_pico_ws2812_spi_rgbw_blink

### SWD

Opposite the micro USB port on the Raspberry Pi Pico are three pins for making an SWD connection.
Currently, support for the Raspberry Pi Pico is missing for many popular debug probe tools, so using a Raspberry Pi Pico's second core as an onboard debugger probably the simplest option.
Refer to the _pico-debug_ section below for instructions.
This section describes the general instructions to use for any debug probe.

Install `probe-run-rp`, required until support lands in upstream [probe-run](https://github.com/knurling-rs/probe-run).

    cargo install --git https://github.com/rp-rs/probe-run --branch main

Change the runner in `.cargo/config.toml` to `probe-run-rp`.

    [target.'cfg(all(target_arch = "arm", target_os = "none"))']
    runner = "probe-run-rp --chip RP2040"

Now just run an example with `cargo run`.

    cargo run --example raspberry_pi_pico_ws2812_spi_rgbw_blink

#### pico-debug

`pico-debug` is a project that uses the second core on the Raspberry Pi Pico a debugger.
Follow the instructions in this section to set this up.

1. Boot the Raspberry Pi Pico in bootloader mode by holding the `BOOTSEL` button while plugging it in.
The Raspberry Pi Pico should show up as a USB drive named `RPI-RP2`.
2. Download the UF2 file containing the `pico-debug` firmware from [here](https://github.com/majbthrd/pico-debug/releases) and save it on the `RPI-RP2` volume.
The `gimmecache` version should work just fine for these examples.
3. You should now be able to connect to the Raspberry Pi Pico as if it is a debug probe.
Follow the instructions in the _SWD_ section to flash and debug an example.

## Examples

The available examples are described in more detail in this section
Examples using the WS2812 SPI driver use the MOSI pin, GPIO7, on the Raspberry Pi Pico.

<dl>
<dt>raspberry_pi_pico_ws2812_spi_rgbw_blink</dt>
<dd>Blinks an <a href="https://www.adafruit.com/product/2867">RGBW NeoPixel Stick</a> containing 8 LED's. The NeoPixels are colored in sets of four where the first is blue, the second is green, the third is red, and the fourth is white.</dd>
</dl>
