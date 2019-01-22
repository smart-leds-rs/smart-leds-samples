#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt;

use apa102_spi as apa102;
use trinket_m0 as hal;

use crate::hal::clock::GenericClockController;
use crate::hal::prelude::*;
use crate::hal::sercom;
use crate::hal::{delay::Delay, CorePeripherals, Peripherals};

use crate::apa102::Apa102;
use smart_leds::Color;
use smart_leds::SmartLedsWrite;

use cortex_m_rt::entry;

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

    let mut pins = crate::hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let spi_pinout = crate::sercom::SPI1Pinout::Dipo3Dopo0 {
        miso: crate::sercom::Sercom1Pad3::pa31(pins.swdio, &mut pins.port),
        mosi: crate::sercom::Sercom1Pad0::pa0(pins.dotstar_di, &mut pins.port),
        sck: crate::sercom::Sercom1Pad1::pa1(pins.dotstar_ci, &mut pins.port),
    };

    let gclk = clocks.gclk0();
    let spi = hal::sercom::SPIMaster1::new(
        &clocks.sercom1_core(&gclk).unwrap(),
        3_000_000.hz(),
        apa102::MODE,
        peripherals.SERCOM1,
        &mut peripherals.PM,
        spi_pinout,
    );

    let mut dotstar = Apa102::new(spi);

    let state0: [Color; 1] = [Color { r: 64, g: 0, b: 0 }];
    let state1: [Color; 1] = [Color { r: 0, g: 64, b: 0 }];
    let state2: [Color; 1] = [Color { r: 0, g: 0, b: 64 }];
    loop {
        dotstar.write(state0.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        dotstar.write(state1.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        dotstar.write(state2.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
    }
}
