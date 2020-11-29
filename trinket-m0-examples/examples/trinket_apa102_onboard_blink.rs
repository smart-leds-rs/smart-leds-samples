#![no_std]
#![no_main]

use panic_halt as _;

use apa102_spi as apa102;
use trinket_m0 as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::TimerCounter;

use crate::apa102::Apa102;
use smart_leds::SmartLedsWrite;
use smart_leds_trait::RGB8;

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

    let mut pins = crate::hal::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let di = pins.dotstar.di.into_push_pull_output(&mut pins.port);
    let ci = pins.dotstar.ci.into_push_pull_output(&mut pins.port);
    let nc = pins.dotstar.nc.into_floating_input(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(5.khz());

    let spi = bitbang_hal::spi::SPI::new(apa102_spi::MODE, nc, di, ci, timer);

    let mut dotstar = Apa102::new(spi);

    let state0: [RGB8; 1] = [RGB8 { r: 64, g: 0, b: 0 }];
    let state1: [RGB8; 1] = [RGB8 { r: 0, g: 64, b: 0 }];
    let state2: [RGB8; 1] = [RGB8 { r: 0, g: 0, b: 64 }];
    loop {
        dotstar.write(state0.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        dotstar.write(state1.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        dotstar.write(state2.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
    }
}
