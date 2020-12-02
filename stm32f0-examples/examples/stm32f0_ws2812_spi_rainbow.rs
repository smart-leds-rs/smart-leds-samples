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

use smart_leds::{brightness, SmartLedsWrite, RGB8};

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
        let mut ws = Ws2812::new(spi);

        const NUM_LEDS: usize = 10;
        let mut data = [RGB8::default(); NUM_LEDS];

        loop {
            for j in 0..(256 * 5) {
                for i in 0..NUM_LEDS {
                    data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                }
                ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                delay.delay_ms(5u8);
            }
        }
    }
    loop {
        continue;
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
