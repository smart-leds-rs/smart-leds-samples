# Uploading the example

* Be in this directory `cd trinket-m0-examples`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step with [cargo-hf2](https://crates.io/crates/cargo-hf2)
```
$ cargo hf2 ----release --example trinket_apa102_onboard_blink
    Finished release [optimized + debuginfo] target(s) in 0.04s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("Trinket M0"))
    Flashing "/[..]/smart-leds-samples/trinket-m0-examples/target/thumbv6m-none-eabi/release/examples/trinket_apa102_onboard_blink"
    Finished in 0.139s
```
