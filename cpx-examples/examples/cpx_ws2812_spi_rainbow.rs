#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt;

use circuit_playground_express as cpx;
use ws2812_spi as ws2812;

use cpx::clock::GenericClockController;
use cpx::pac::{Peripherals, CorePeripherals};
use cpx::delay::Delay;
use cpx::time::U32Ext;
use cpx::sercom::PadPin;
use embedded_hal::blocking::delay::DelayMs;
use cortex_m_rt::entry;

use smart_leds::SmartLedsWrite;
use smart_leds_trait::RGB8;
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
    let mut pins = cpx::Pins::new(peripherals.PORT);

    let miso = pins.sda.into_pad(&mut pins.port);
    let mosi = pins.neopixel.into_pad(&mut pins.port);
    let sck = pins.scl.into_pad(&mut pins.port);

    let gclk = clocks.gclk0();
    let spi = cpx::sercom::SPIMaster5::new(
        &clocks.sercom5_core(&gclk).unwrap(),
        3000.khz(),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        },
        peripherals.SERCOM5,
        &mut peripherals.PM,
        (miso, mosi, sck),
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    const NUM_LEDS: usize = 10;
    let mut data = [RGB8::default().into(); NUM_LEDS];

    let mut neopixel = ws2812::Ws2812::new(spi);

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
fn wheel(mut wheel_pos: u8) -> RGB8 {
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
