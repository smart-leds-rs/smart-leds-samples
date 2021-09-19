# BBC micro:bit v2 Examples

These examples use the [nrf-rs/microbit](https://github.com/nrf-rs/microbit/) HAL library for the BBC micro:bit v2.
The code is flashed to the micro:bit using [probe-rs](https://probe.rs/).
Please follow the [Probe Setup](https://probe.rs/docs/getting-started/probe-setup/) instructions for probe-rs before proceeding.

Install the necessary dependencies.

    cargo install cargo-embed

Connect your computer to the BBC micro:bit v2 via the micro USB port.

Now flash an example to the micro:bit using [cargo-embed](https://probe.rs/docs/tools/cargo-embed/).

    cargo embed --example microbit_v2_ws2812_spi_blink

## Examples

The available examples are described in more detail in this section
Examples using the WS2812 SPI driver use the MOSI pin, pin #15, on the BBC micro:bit v2.


<dl>
  <dt>microbit_v2_ws2812_spi_blink</dt>
  <dd>Blinks a Neopixel chain of thirty LED's on and off in one second intervals. The Neopixels are colored in sets of three where the first is blue, the second is green, and the third is red.</dd>
</dl>

