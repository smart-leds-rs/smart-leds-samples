#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use stm32f0xx_hal as hal;
use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::hal::stm32;
use crate::ws2812::Ws2812;
use cortex_m::peripheral::Peripherals;

use smart_leds::{SmartLedsWrite, RGB8};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_default!();

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
            ws2812::MODE,
            3_000_000.hz(),
            &mut rcc,
        );

        let mut data: [RGB8; 3] = [RGB8::default(); 3];
        let empty: [RGB8; 3] = [RGB8::default(); 3];
        let mut ws = Ws2812::new(spi);
        loop {
            data[0] = RGB8 {
                r: 0,
                g: 0,
                b: 0x10,
            };
            data[1] = RGB8 {
                r: 0,
                g: 0x10,
                b: 0,
            };
            data[2] = RGB8 {
                r: 0x10,
                g: 0,
                b: 0,
            };
            ws.write(data.iter().cloned()).unwrap();
            delay.delay_ms(1000 as u16);
            ws.write(empty.iter().cloned()).unwrap();
            delay.delay_ms(1000 as u16);
        }
    }
    loop {
        continue;
    }
}
