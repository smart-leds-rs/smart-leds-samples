#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;
use rtt_target::rtt_init_default;

use longan_nano::hal::{delay::McycleDelay, pac, prelude::*, spi::Spi};

use embedded_hal::spi;
use embedded_time::duration::Milliseconds;
use embedded_time::fixed_point::FixedPoint;
use smart_leds::{SmartLedsWrite, White, RGBW};
use ws2812_spi::Ws2812;

#[entry]
fn main() -> ! {
    rtt_init_default!();

    const DELAY: Milliseconds<u32> = Milliseconds::<u32>(1_000);
    // This example uses an RGBW NeoPixel Stick with 8 RGBW NeoPixels.
    const NUM_LEDS: usize = 8;
    debug_assert_ne!(NUM_LEDS, 0);

    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);
    let gpioa = dp.GPIOA.split(&mut rcu);

    let mut delay = McycleDelay::new(&rcu.clocks);

    let sck = gpioa.pa5.into_alternate_push_pull();
    let miso = gpioa.pa6.into_floating_input();
    // The SPI MOSI pin here is pin A7 on the Longan Nano.
    let mosi = gpioa.pa7.into_alternate_push_pull();
    let spi = Spi::spi0(
        dp.SPI0,
        (sck, miso, mosi),
        &mut afio,
        spi::MODE_0,
        3_000_000.hz(),
        &mut rcu,
    );

    let mut ws = Ws2812::new_sk6812w(spi);

    let mut data: [RGBW<u8>; NUM_LEDS] = [RGBW::default(); NUM_LEDS];
    let empty: [RGBW<u8>; NUM_LEDS] = [RGBW::default(); NUM_LEDS];

    // Blink the LED's in a blue-green-red-white pattern.
    for led in data.iter_mut().step_by(4) {
        led.b = 0x10;
    }

    if NUM_LEDS > 1 {
        for led in data.iter_mut().skip(1).step_by(4) {
            led.g = 0x10;
        }
    }

    if NUM_LEDS > 2 {
        for led in data.iter_mut().skip(2).step_by(4) {
            led.r = 0x10;
        }
    }

    if NUM_LEDS > 3 {
        for led in data.iter_mut().skip(3).step_by(4) {
            led.a = White(0x10);
        }
    }

    loop {
        ws.write(data.iter().cloned()).unwrap();
        delay.delay_ms(DELAY.integer() as u16);
        ws.write(empty.iter().cloned()).unwrap();
        delay.delay_ms(DELAY.integer() as u16);
    }
}
