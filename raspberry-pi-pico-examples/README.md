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
Support for the Raspberry Pi Pico is missing for many popular debug probe tools currently, so using a second Raspberry Pi Pico as the debug probe works best.
Refer to the _picoprobe_ section below for instructions.
This section describes the general instructions to use any debug probe.

Install `probe-run-rp`, required until support lands in upstream [probe-run](https://github.com/knurling-rs/probe-run).

    cargo install --git https://github.com/rp-rs/probe-run --branch main

Change the runner in `.cargo/config.toml` to `probe-run-rp`.

    [target.'cfg(all(target_arch = "arm", target_os = "none"))']
    runner = "probe-run-rp --chip RP2040"

Now just run an example with `cargo run`.

    cargo run --release --example raspberry_pi_pico_ws2812_spi_rgbw_blink

#### picoprobe

The `picoprobe` is a Raspberry Pi Pico configured to run as a debug probe.
To use a `picoprobe` to flash and debug the examples, follow these instructions.

0. Now, boot the Raspberry Pi Pico in bootloader mode by holding the `BOOTSEL` button while plugging it in.
0. Download the UF2 file containing the `picoprobe` firmware.
0. Download and install the `picoprobe` firmware: https://github.com/majbthrd/DapperMime/releases/download/20210225/raspberry_pi_pico-DapperMime.uf2.
0. Wire the two Picos together according to the instructions https://datasheets.raspberrypi.org/pico/getting-started-with-pico.pdf#%5B%7B%22num%22%3A60%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D[here].

## Examples

The available examples are described in more detail in this section
Examples using the WS2812 SPI driver use the MOSI pin, GPIO7, on the Raspberry Pi Pico.

<dl>
<dt>raspberry_pi_pico_ws2812_spi_rgbw_blink</dt>
<dd>Blinks an <a href="https://www.adafruit.com/product/2867">RGBW NeoPixel Stick</a> containing 8 LED's. The NeoPixels are colored in sets of four where the first is blue, the second is green, the third is red, and the fourth is white.</dd>
</dl>
