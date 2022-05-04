#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use microbit::{
    hal::{
        gpio::{p0::Parts, Level},
        prelude::*,
        spi, Timer,
    },
    Peripherals,
};

use embedded_time::duration::*;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

#[entry]
fn main() -> ! {
    rtt_init_default!();

    const DELAY: Milliseconds<u32> = Milliseconds::<u32>(1_000);

    // This example uses a Neopixel Strip with 30 RGB Neopixels.
    const NUM_LEDS: usize = 30;
    debug_assert_ne!(NUM_LEDS, 0);

    let dp = Peripherals::take().unwrap();
    let mut delay = Timer::new(dp.TIMER0);
    let port0 = Parts::new(dp.P0);
    let sck = port0.p0_17.into_push_pull_output(Level::Low).degrade();

    // The SPI MOSI pin is pin 15 on the BBC micro:bit.
    let mosi = port0.p0_13.into_push_pull_output(Level::Low).degrade();

    let miso = port0.p0_01.into_floating_input().degrade();
    let pins = spi::Pins {
        sck,
        miso: Some(miso),
        mosi: Some(mosi),
    };
    let spi = spi::Spi::new(dp.SPI0, pins, spi::Frequency::M4, spi::MODE_0);

    let mut ws = Ws2812::new(spi);

    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];

    // Blink the LED's in a blue-green-red pattern.
    for led in data.iter_mut().step_by(3) {
        led.b = 0x10;
    }

    if NUM_LEDS > 1 {
        for led in data.iter_mut().skip(1).step_by(3) {
            led.g = 0x10;
        }
    }

    if NUM_LEDS > 2 {
        for led in data.iter_mut().skip(2).step_by(3) {
            led.r = 0x10;
        }
    }

    loop {
        ws.write(data.iter().cloned()).unwrap();
        delay.delay_ms(DELAY.integer() as u16);
        ws.write(empty.iter().cloned()).unwrap();
        delay.delay_ms(DELAY.integer() as u16);
    }
}
