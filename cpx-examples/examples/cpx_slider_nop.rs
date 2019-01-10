#![no_std]
#![no_main]

#[macro_use(entry)]
extern crate cortex_m_rt;
extern crate cortex_m;

extern crate panic_halt;

extern crate circuit_playground_express as hal;
extern crate ws2812_nop_samd21 as ws2812;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{CorePeripherals, Peripherals};

use smart_leds_trait::Color;
use smart_leds_trait::SmartLedsWrite;
use ws2812::Ws2812;

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

    let mut pins = circuit_playground_express::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = Ws2812::new(neopixel_pin);

    const MAX: usize = 10;
    const COLOR1: (u8, u8, u8) = (0x00, 0xc3 / 5, 0x36 / 5);
    const COLOR2: (u8, u8, u8) = (0x00, 0x24 / 5, 0xb0 / 5);
    let mut data = [Color::default(); MAX];
    let mut main = 0;
    let mut up = true;

    loop {
        for i in 0..MAX {
            let distance = (main as i32 - i as i32).abs() as u8;
            let c1 = (
                COLOR1.0 as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                COLOR1.1 as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                COLOR1.2 as u32 * (MAX as u32 - distance as u32) / MAX as u32,
            );
            let c2 = (
                COLOR2.0 as u32 * distance as u32 / MAX as u32,
                COLOR2.1 as u32 * distance as u32 / MAX as u32,
                COLOR2.2 as u32 * distance as u32 / MAX as u32,
            );
            let ct = (
                (c1.0 + c2.0) as u8,
                (c1.1 + c2.1) as u8,
                (c1.2 + c2.2) as u8,
            );
            data[i] = ct.into();
        }
        if up {
            if main == MAX - 1 {
                up = false;
                main -= 2;
            }
            main += 1;
        } else {
            if main == 0 {
                up = true;
                main += 2;
            }
            main -= 1;
        }
        neopixel.write(data.iter().cloned()).unwrap();
        delay.delay_ms(100 as u16);
    }
}
