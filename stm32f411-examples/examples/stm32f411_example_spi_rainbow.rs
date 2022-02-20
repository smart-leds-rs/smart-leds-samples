#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::pac;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::ws2812::Ws2812;

use smart_leds::{brightness, SmartLedsWrite, RGB8};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Constrain clocking registers
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
        
        let mut delay = Delay::new(cp.SYST, &clocks);

        // GPIOA used for SPI, GPIOC for onboard led
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // turn onboard led on
        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();

        // Configure pins for SPI, PA7 is output going to data line of leds
        let sck1 = gpioa.pa5.into_alternate();
        let miso1 = gpioa.pa6.into_alternate();
        let mosi1 = gpioa.pa7.into_alternate();

        // SPI1 with 3Mhz
        let spi = Spi::new(dp.SPI1, (sck1, miso1, mosi1), ws2812::MODE, 3_000_000.hz(), clocks);

        let mut ws = Ws2812::new(spi);

        const LED_NUM: usize = 60;
        let mut data = [RGB8::default(); LED_NUM];

        loop {
            for j in 0..(256 * 5) {
                for i in 0..LED_NUM {
                    data[i] = wheel((((i * 256) as u16 / LED_NUM as u16 + j as u16) & 255) as u8);
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