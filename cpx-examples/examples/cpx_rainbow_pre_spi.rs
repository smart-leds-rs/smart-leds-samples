#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt;

use circuit_playground_express as hal;
use ws2812_spi as ws2812;

use crate::hal::clock::GenericClockController;
use crate::hal::{Peripherals, CorePeripherals};
use crate::hal::delay::Delay;
use crate::hal::sercom::PadPin;
use crate::hal::time::U32Ext;
use crate::ws2812::prerendered::Timing;
use embedded_hal::blocking::delay::DelayMs;
use cortex_m_rt::entry;

use smart_leds_trait::SmartLedsWrite;
use smart_leds_trait::Color;
use smart_leds::brightness;

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
    let mut pins = hal::Pins::new(peripherals.PORT);

    let spi_pinout = hal::sercom::SPI5Pinout::Dipo0Dopo2 {
        miso: pins.sda.into_pad(&mut pins.port),
        mosi: pins.neopixel.into_pad(&mut pins.port),
        sck: pins.scl.into_pad(&mut pins.port) 
    };

    let gclk = clocks.gclk0();
    let spi = hal::sercom::SPIMaster5::new(
        &clocks.sercom5_core(&gclk).unwrap(),
        2_000_000u32.hz(),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        },
        peripherals.SERCOM5,
        &mut peripherals.PM,
        spi_pinout,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    const NUM_LEDS: usize = 10;
    let mut data = [Color::default().into(); NUM_LEDS];
    let mut rendered_data = [0; NUM_LEDS * 3 * 5];

    let mut neopixel = ws2812::prerendered::Ws2812::new(spi, Timing::new(2_000_000).unwrap(), &mut rendered_data);

    loop {
        for j in 0..(256*5) {
            for i in 0..NUM_LEDS {
                data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            neopixel.write(brightness(data.iter().cloned(), 32)).unwrap();
            delay.delay_ms(5u8);
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> Color {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into()
    }
    if wheel_pos < 170 {
        wheel_pos -=85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into()
    }
    wheel_pos -= 170;
    (wheel_pos*3, 255 - wheel_pos * 3, 0).into()
}
