#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
use panic_halt as _;

use arduino_leonardo::prelude::*;
use arduino_leonardo::spi;

use ws2812_spi as ws2812;

use crate::ws2812::prerendered::Ws2812;
use smart_leds::{SmartLedsWrite, RGB8};

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);
    // Create SPI interface.
    let (spi, _) = spi::Spi::new(
        dp.SPI,
        pins.sck.into_output(&mut pins.ddr),
        pins.mosi.into_output(&mut pins.ddr),
        pins.miso.into_pull_up_input(&mut pins.ddr),
        pins.led_rx.into_output(&mut pins.ddr),
        spi::Settings {
            clock: spi::SerialClockRate::OscfOver8,
            ..Default::default()
        },
    );

    let mut output_buffer = [0; 20 + (3 * 12)];
    let mut data: [RGB8; 3] = [RGB8::default(); 3];
    let empty: [RGB8; 3] = [RGB8::default(); 3];
    let mut ws = Ws2812::new(spi, &mut output_buffer);
    loop {
        data[0] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[1] = RGB8 {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[2] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };
        ws.write(data.iter().cloned()).unwrap();
        arduino_leonardo::delay_ms(1000 as u16);
        ws.write(empty.iter().cloned()).unwrap();
        arduino_leonardo::delay_ms(1000 as u16);
    }

}
