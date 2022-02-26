# STM32F411 (blackpill) example

This project is an example of running ws2812b ledstrip on STM32F411CE also know as blackpill. Implementation uses SPI1, so data line needs to be connected to pin A7, this can be changed according to pinout.
Code also relies on [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal).

A more detailed guide on setting up toolchain can be found [here](https://github.com/blaz-r/STM32F411-rust-neopixel). 

Following is a quick-start guide in case you are unfamiliar with building and running code for stm32.

## Important

To get this to work, you'll need to cross-compile the code and compile it with --release tag. To flash and run the code, you can either use probe-run or GDB with openOCD. More about running the project below.

## Toolchain

This code needs to be cross-compiled to work on STM32 microcontroller, which is an Arm Cortex M4 with FPU core. More info about the chip itself can be found in [Reference manual PDF](https://www.st.com/resource/en/reference_manual/dm00119316-stm32f411xc-e-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf).


In order to cross-compile, you'll need to add target used for this microcontroller <b>thumbv7em-none-eabihf</b>, using following command
```
rustup target add thumbv7em-none-eabihf
```

As stated in The Embedded Rust Book, I also installed cargo-binutils and cargo-generate. More details can be found [here](https://docs.rust-embedded.org/book/intro/install.html).

At this point you can either use probe-run or GDB with openOCD to flash the code to microcontroller. Following are the setup instructions for either option, you don't need both.

#### probe-run

Probe run enables you to run the code on microcontroller, as if it was normal rust code. More info about it can be found in [probe-run repo](https://github.com/knurling-rs/probe-run).

To install, run following command:
```
cargo install probe-run
```
If that doesn't work, refer to repo linked above.
You might also need a ST-LINK driver, at least on Windows if you don't already have one.

Once you have probe-run installed you can run code as described in one of the sections bellow, config file already contains setting for probe-run and stm32f411CEU blackpill.

#### GDB & OpenOCD

Everything about installation and details about tools are explained in [The Embedded Rust Book, section Tooling and Installation
](https://docs.rust-embedded.org/book/intro/tooling.html)

On the same page you can also find OS-Specific instructions for installing following needed tools:
- GDB for Arm programs
- OpenOCD to work with ST-LINK
- ST-LINK usb driver if needed

Once all of these are installed, you can verify installation is working with following command:
```
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
```
Something along the following output should be produced:
```
...
Info : clock speed 2000 kHz
Info : STLINK V2J37S7 (API v2) VID:PID 0483:3748
Info : Target voltage: 3.240957
Info : stm32f4x.cpu: Cortex-M4 r0p1 processor detected
Info : stm32f4x.cpu: target has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
```
If it is not working, please refer to [following page](https://docs.rust-embedded.org/book/intro/install/verify.html).

### Project settings

For project to work correctly with HAL and given microcontroller, you'll also need <b>[.cargo/config](/stm32f411-examples/.cargo/config)</b> and <b>[memory.x](/stm32f411-examples/memory.x)</b> files. These are already present in this project and set for this microcontroller, more details can found [here](https://github.com/blaz-r/STM32F411-rust-neopixel).

### Building project

With all properly configured, project can be simply built using following command
```
cargo build --release --example stm32f411_example_spi_rainbow
```
--release flag here was needed in my case, without this flag, the project didn't work correctly.

### Running project with probe-run

To run project with probe-run, use the following instruction from inside project folder:
```
cargo run --release --example stm32f411_example_spi_rainbow
```
Probe runner, with help of config file, will take care of the rest. As with normal rust, using <i>run</i> will build the project, then flash and run it on microcontroller.

### Running project with GDB & OpenOCD

To load and run the project on microcontroller, you'll need OpenOCD and GDB for Arm, which you can install as described above.

OpenOCD serves as an intermediate between GDB and ST-LINK and GDB itself is used for debugging and loading the code to microcontroller.

Following are the command I use on my windows setup, it should be very similar for linux:
- first run openOCD: 
    ```
    openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
    ```
- then run GDB in <b>separate terminal</b>. Following is command executed in root of project:
    ```
    arm-none-eabi-gdb -q .\target\thumbv7em-none-eabihf\release\examples\stm32f411_example_spi_rainbow
    ```

This now opens GDB. It can be used to debug code, or in this case just load it and run the project.

First write te following to connect with already running openOCD:
```
target remote :3333
```

Then to load the project:
```
load
```

And at this point, if you just want to run it:
```
continue
```

Now the onboard led should turn on and the ledstrip should have a rainbow animation.