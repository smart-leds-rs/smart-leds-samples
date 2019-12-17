#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;
use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::hal::stm32;
use crate::ws2812::prerendered::{Timing, Ws2812};
use cortex_m::peripheral::Peripherals;

use smart_leds::{SmartLedsWrite, RGB8};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        // Constrain clocking registers
        let mut flash = p.FLASH;
        // Configure clock to 8 MHz (i.e. the default) and freeze it
        let mut rcc = p.RCC.configure().sysclk(32.mhz()).freeze(&mut flash);

        let gpioa = p.GPIOA.split(&mut rcc);

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &rcc);

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
            ws2812::MODE,
            4_000_000.hz(),
            &mut rcc,
        );
        const MAX: usize = 8;
        const COLOR1: RGB8 = RGB8 {
            r: 0x00,
            g: 0xc3 / 5,
            b: 0x36 / 5,
        };
        const COLOR2: RGB8 = RGB8 {
            r: 0x00,
            g: 0x24 / 5,
            b: 0xb0 / 5,
        };
        let mut data: [RGB8; MAX] = [(0, 0, 0).into(); MAX];
        let mut main = 0;
        let mut render_data = [0; MAX * 3 * 5];
        let mut ws = Ws2812::new(spi, Timing::new(4000000).unwrap(), &mut render_data);
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
            ws.write(data.iter().cloned()).unwrap();
            delay.delay_ms(100 as u16);
        }
    }
    loop {
        continue;
    }
}
