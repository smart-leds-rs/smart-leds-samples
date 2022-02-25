#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::pac;
use crate::hal::gpio::NoPin;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::ws2812::Ws2812;

use smart_leds::{gamma, SmartLedsWrite, RGB8, hsv::Hsv, hsv::hsv2rgb};

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

        // Configure pins for SPI
        let sck1 = gpioa.pa5.into_alternate();
        let miso1 = NoPin;                          // miso not needed
        let mosi1 = gpioa.pa7.into_alternate();     // PA7 is output going to data line of leds

        // SPI1 with 3Mhz
        let spi = Spi::new(dp.SPI1, (sck1, miso1, mosi1), ws2812::MODE, 3_000_000.hz(), clocks);

        let mut ws = Ws2812::new(spi);

        const LED_NUM: usize = 60;
        let mut data = [RGB8::default(); LED_NUM];

        loop {
            for j in 0..256 {
                for i in 0..LED_NUM {
                    // rainbow cycle using HSV, where hue goes through all colors in circle
                    // value sets the brightness
                    let hsv = Hsv{hue: ((i * 3 + j) % 256) as u8, sat: 255, val: 100};

                    data[i] = hsv2rgb(hsv);
                }
                // before writing, apply gamma correction for nicer rainbow
                ws.write(gamma(data.iter().cloned())).unwrap();
                delay.delay_ms(10u8);
            }
        }
    }
    loop {
        continue;
    }
}