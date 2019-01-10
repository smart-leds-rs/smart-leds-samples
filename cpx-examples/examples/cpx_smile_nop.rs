#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
extern crate circuit_playground_express as hal;
extern crate ws2812_nop_samd21 as ws2812;

use hal::clock::GenericClockController;
use hal::Peripherals;

use smart_leds::colors::YELLOW;
use smart_leds_trait::Color;
use smart_leds_trait::SmartLedsWrite;
use ws2812::Ws2812;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let _clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = circuit_playground_express::Pins::new(peripherals.PORT);
    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = Ws2812::new(neopixel_pin);

    let off = Color::default();
    let smile = [
        YELLOW, off, YELLOW, YELLOW, YELLOW, YELLOW, YELLOW, YELLOW, off, YELLOW,
    ];
    neopixel.write(smile.iter().cloned()).unwrap();
    loop {}
}
