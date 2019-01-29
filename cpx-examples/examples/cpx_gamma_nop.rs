#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt;

use circuit_playground_express as hal;
use ws2812_nop_samd21 as ws2812;

use crate::hal::clock::GenericClockController;
use crate::hal::{Peripherals, CorePeripherals};
use crate::hal::delay::Delay;
use embedded_hal::blocking::delay::DelayMs;
use cortex_m_rt::entry;

use smart_leds_trait::SmartLedsWrite;
use smart_leds_trait::Color;
use smart_leds::colors::ORANGE;
use smart_leds::gamma;
use smart_leds::brightness;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = ws2812::Ws2812::new(neopixel_pin);

    let mut delay = Delay::new(core.SYST, &mut clocks);
    const NUM_LEDS: usize = 10;
    let mut data = [Color::default(); NUM_LEDS];

    loop {
        for i in 0..NUM_LEDS {
            data[i] = ORANGE;
        }
        neopixel.write(brightness(data.iter().cloned(), 32)).unwrap();
        delay.delay_ms(1000u16);

        for i in 0..NUM_LEDS {
            data[i] = ORANGE;
        }
        neopixel.write(brightness(gamma(data.iter().cloned()), 32)).unwrap();
        delay.delay_ms(1000u16);
    }
}
