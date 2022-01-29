# Raspberry Pi Examples

This is an example that can be used on multiple Raspberry Pi models.
This uses the [ws281x-rpi driver](https://github.com/nathansamson/ws281x-rpi)
which can be used within Raspbian OS which means the rust std library can be used.

## Program

The example uses GPIO pin 10 (SPI mode) to interact with the LEDs. Check the
[documentation](https://github.com/jgarff/rpi_ws281x#gpio-usage) and
[limitations](https://github.com/jgarff/rpi_ws281x#limitations) about this.
Model specific OS configuration might be necessary to run this example.
Depending on the mode used root access might be required as well.

You can easily change the pin number in the example program to use other modes
(PWM, PCM) if the default mode (SPI) does not work for your use case.
