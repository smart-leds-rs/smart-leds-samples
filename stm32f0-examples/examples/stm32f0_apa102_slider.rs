#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use apa102_spi as apa102;
use stm32f0xx_hal as hal;

use crate::apa102::Apa102;
use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::hal::stm32;
use cortex_m::peripheral::Peripherals;

use smart_leds::{RGB8, SmartLedsWrite};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        // Constrain clocking registers
        let mut flash = p.FLASH;
        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut flash);
        let gpioa = p.GPIOA.split(&mut rcc);

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &mut rcc);

        // Configure pins for SPI
        let (sck, miso, mosi) = cortex_m::interrupt::free(move |cs| {
            (
                gpioa.pa5.into_alternate_af0(cs),
                gpioa.pa6.into_alternate_af0(cs),
                gpioa.pa7.into_alternate_af0(cs),
            )
        });

        // Configure SPI with 3Mhz rate
        let spi = Spi::spi1(
            p.SPI1,
            (sck, miso, mosi),
            apa102::MODE,
            1_000_000.hz(),
            &mut rcc,
        );
        const MAX: usize = 8;
        const COLOR1: RGB8 = RGB8 {
            r: 0x00,
            g: 0xc3,
            b: 0x36,
        };
        const COLOR2: RGB8 = RGB8 {
            r: 0x00,
            g: 0x24,
            b: 0xb0,
        };
        let mut data = [(0, 0, 0).into(); MAX];
        let mut main = 0;
        let mut apa = Apa102::new(spi);
        let mut up = true;
        loop {
            for i in 0..MAX {
                let distance = (main as i32 - i as i32).abs() as u8;
                let c1 = (
                    COLOR1.r as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                    COLOR1.g as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                    COLOR1.b as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                );
                let c2 = (
                    COLOR2.r as u32 * distance as u32 / MAX as u32,
                    COLOR2.g as u32 * distance as u32 / MAX as u32,
                    COLOR2.b as u32 * distance as u32 / MAX as u32,
                );
                let ct = (
                    (c1.0 + c2.0) as u8,
                    (c1.1 + c2.1) as u8,
                    (c1.2 + c2.2) as u8,
                )
                    .into();
                data[i] = ct;
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
            apa.write(data.iter().cloned()).unwrap();
            delay.delay_ms(100 as u16);
        }
    }
    loop {
        continue;
    }
}
