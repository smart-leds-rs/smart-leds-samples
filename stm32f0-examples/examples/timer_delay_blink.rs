#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;
use ws2812_timer_delay as ws2812;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::stm32;
use crate::hal::time::*;
use crate::hal::timers::*;
use crate::ws2812::Ws2812;
use cortex_m::peripheral::Peripherals;

use smart_leds::{Color, SmartLedsWrite};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpioa = p.GPIOA.split();

        /* (Re-)configure PA7 as output */
        let mut ws_data_pin = gpioa.pa7.into_push_pull_output_hs();

        // Constrain clocking registers
        let rcc = p.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let timer = Timer::tim1(p.TIM1, MegaHertz(3), clocks);

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        let mut ws = Ws2812::new(timer, &mut ws_data_pin);
        let mut data: [Color; 3] = [Color::default(); 3];
        let empty: [Color; 3] = [Color::default(); 3];

        data[0] = Color {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[1] = Color {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[2] = Color {
            r: 0x10,
            g: 0,
            b: 0,
        };

        loop {
            ws.write(data.iter().cloned()).unwrap();
            delay.delay_ms(10 as u16);
            ws.write(empty.iter().cloned()).unwrap();
            delay.delay_ms(10 as u16);
        }
    }
    loop {
        continue;
    }
}
