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

    let mut data: [RGB8; 10] = [RGB8::default(); 10];
    let empty: [RGB8; 10] = [RGB8::default(); 10];
    let mut ws = ws2812::Ws2812::new(spi);
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
        data[3] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[4] = RGB8 {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[5] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };
        data[6] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[7] = RGB8 {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[8] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };
        data[9] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };

        ws.write(data.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        ws.write(empty.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
    }
}
