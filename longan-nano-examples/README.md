# Longan Nano Examples

These examples use the [riscv-rust/longan-nano](https://github.com/riscv-rust/longan-nano) BSP for the SiPEED Longan Nano v1.0.
The code is flashed to the Longan Nano using [OpenOCD](https://openocd.org/) and GDB.
The [RISC-V GNU Compiler Toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) is required, so make sure to build and install it before following the instructions below.

## Flash

The easiest ways to flash the examples to the Longan Nano are over the USB-C connection using `dfu-util` and using the SiPEED RISC-V Debugger.

### dfu-util

The [dfu-util](http://dfu-util.sourceforge.net/) tool can be used to flash the Longan Nano directly over the USB-C port.

    sudo dnf -y install dfu-util

Build the example first.

    cargo build --example longan_nano_ws2812_spi_rgbw_blink

Then convert it to a `.bin` binary.

    riscv64-unknown-elf-objcopy -O binary target/riscv32imac-unknown-none-elf/debug/examples/longan_nano_ws2812_spi_rgbw_blink firmware.bin

Attach the Longan Nano's USB-C port to your computer.
Put the Longan Nano in bootloader mode by holding the `BOOT0` button pressed while pressing and releasing the reset button.
Now flash the `.bin` file onto the Longan Nano using the `dfu-util` program.

    sudo dfu-util -a 0 -s 0x08000000:leave -D firmware.bin

### SiPEED RISC-V Debugger

The [SiPEED RISC-V Debugger](https://www.seeedstudio.com/Sipeed-USB-JTAG-TTL-RISC-V-Debugger-ST-Link-V2-STM8-STM32-Simulator-p-2910.html) can be used to flash and debug the examples over a JTAG connection.
First, attach the debug probe to the JTAG pins opposite the USB-C port of the Longan Nano.

With a debug probe connected, start OpenOCD.

    openocd -f sipeed-jtag.cfg -f target/gd32vf103.cfg

Now, launch GDB using the `cargo run` subcommand and the desired example.

    cargo run --example longan_nano_ws2812_spi_rgbw_blink

## Examples

The available examples are described in more detail in this section
Examples using the WS2812 SPI driver use the MOSI pin, A7, on the Longan Nano v1.0.

<dl>
  <dt>longan_nano_ws2812_spi_rgbw_blink</dt>
  <dd>Blinks an <a href="https://www.adafruit.com/product/2867">RGBW NeoPixel Stick</a> containing 8 LED's. The NeoPixels are colored in sets of four where the first is blue, the second is green, the third is red, and the fourth is white.</dd>
</dl>
