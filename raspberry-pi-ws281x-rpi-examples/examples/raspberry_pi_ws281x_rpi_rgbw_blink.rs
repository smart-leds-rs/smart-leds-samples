use std::{thread, time};

use smart_leds::{SmartLedsWrite, RGB8};
use ws281x_rpi::Ws2812Rpi;

fn main() {
    println!("Program start");

    // GPIO Pin 10 is SPI
    // Other modes and PINs are available depending on the Raspberry Pi revision
    // Additional OS configuration might be needed for any mode.
    // Check https://github.com/jgarff/rpi_ws281x for more information.
    const PIN: i32 = 10;
    const NUM_LEDS: usize = 8;
    const DELAY: time::Duration = time::Duration::from_millis(1000);

    let mut ws = Ws2812Rpi::new(NUM_LEDS as i32, PIN).unwrap();

    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];

    // Blink the LED's in a blue-green-red-white pattern.
    for led in data.iter_mut().step_by(4) {
        led.b = 32;
    }

    if NUM_LEDS > 1 {
        for led in data.iter_mut().skip(1).step_by(4) {
            led.g = 32;
        }
    }

    if NUM_LEDS > 2 {
        for led in data.iter_mut().skip(2).step_by(4) {
            led.r = 32;
        }
    }

    if NUM_LEDS > 3 {
        for led in data.iter_mut().skip(3).step_by(4) {
            led.r = 32;
            led.g = 32;
            led.b = 32;
        }
    }

    loop {
        // On
        println!("LEDS on");
        ws.write(data.iter().cloned()).unwrap();
        thread::sleep(DELAY);

        // Off
        println!("LEDS off");
        ws.write(empty.iter().cloned()).unwrap();
        thread::sleep(DELAY);
    }
}
