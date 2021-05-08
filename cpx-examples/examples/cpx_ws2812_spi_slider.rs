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
    let mut data: [RGB8; NUM_LEDS] = [(0, 0, 0).into(); NUM_LEDS];
    let mut main = 0;
    let mut ws = ws2812::Ws2812::new(spi);
    let mut up = true;
    loop {
        for i in 0..NUM_LEDS {
            let distance = (main as i32 - i as i32).abs() as u8;
            let c1 = (
                COLOR1.r as u32 * (NUM_LEDS as u32 - distance as u32) / NUM_LEDS as u32,
                COLOR1.g as u32 * (NUM_LEDS as u32 - distance as u32) / NUM_LEDS as u32,
                COLOR1.b as u32 * (NUM_LEDS as u32 - distance as u32) / NUM_LEDS as u32,
            );
            let c2 = (
                COLOR2.r as u32 * distance as u32 / NUM_LEDS as u32,
                COLOR2.g as u32 * distance as u32 / NUM_LEDS as u32,
                COLOR2.b as u32 * distance as u32 / NUM_LEDS as u32,
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
            if main == NUM_LEDS - 1 {
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
