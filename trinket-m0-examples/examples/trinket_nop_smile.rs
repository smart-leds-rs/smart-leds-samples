#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[allow(unused)]
use panic_halt;

use trinket_m0 as hal;
use ws2812_nop_samd21 as ws2812;

use crate::hal::clock::GenericClockController;
use crate::hal::Peripherals;

use crate::ws2812::Ws2812;
use smart_leds::colors::YELLOW;
use smart_leds::Color;
use smart_leds::SmartLedsWrite;

entry!(main);

fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let _clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = trinket_m0::Pins::new(peripherals.PORT);
    let neopixel_pin = pins.d4.into_push_pull_output(&mut pins.port);
    let mut neopixel = Ws2812::new(neopixel_pin);

    let off = Color::default();
    let smile = [
        YELLOW, off, YELLOW, YELLOW, YELLOW, YELLOW, YELLOW, YELLOW, off, YELLOW,
    ];
    neopixel.write(smile.iter().cloned()).unwrap();
    loop {}
}
