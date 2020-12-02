#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use lpc8xx_hal as hal;
use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::spi;
use crate::hal::{cortex_m_rt::entry, CorePeripherals, Peripherals};
use crate::ws2812::Ws2812;

use smart_leds::{SmartLedsWrite, RGB8};

#[entry]
fn main() -> ! {
    rtt_init_default!();

    if let (Some(dp), Some(cp)) = (Peripherals::take(), CorePeripherals::take()) {
        // Initialize the APIs of the peripherals we need.
        let mut delay = Delay::new(cp.SYST);

        let swm = dp.SWM.split();
        let mut syscon = dp.SYSCON.split();

        let mut handle = swm.handle.enable(&mut syscon.handle); // SWM isn't enabled by default on LPC845.

        let sck_pin = dp.pins.pio0_13.into_swm_pin();
        let mosi_pin = dp.pins.pio0_14.into_swm_pin();
        let miso_pin = dp.pins.pio0_15.into_swm_pin();

        let (spi0_sck, _) = swm.movable_functions.spi0_sck.assign(sck_pin, &mut handle);
        let (spi0_mosi, _) = swm
            .movable_functions
            .spi0_mosi
            .assign(mosi_pin, &mut handle);
        let (spi0_miso, _) = swm
            .movable_functions
            .spi0_miso
            .assign(miso_pin, &mut handle);
        let spi_clock = spi::Clock::new(&syscon.iosc, 5);

        // Enable SPI0
        let spi = dp.SPI0.enable_as_master(
            &spi_clock,
            &mut syscon.handle,
            ws2812::MODE,
            spi0_sck,
            spi0_mosi,
            spi0_miso,
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
