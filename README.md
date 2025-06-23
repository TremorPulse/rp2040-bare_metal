# An Analysis of the Rust Programming Language for Embedded Systems

---

**IMPORTANT:**  
VSCode will attempt to automatically build the files when they are placed in the project.

---

## Hardware Setup

### General Notes:
1. Refer to the `#define` pre-processed sections in the code, the breadboard schematic, and the report for GPIO pin listings.
2. Seven-segment display connections can be ignored.

---

### Transmitter Pico (TX)

| Function        | Pico Pin   | Connected To                        |
|----------------|------------|-------------------------------------|
| GND            | Pin 23     | Negative terminal of the buzzer (–) |
| GPIO21         | Pin 27     | Positive terminal of the buzzer (+) |
| GPIO16         | Pin 21     | Positive terminal of the button (+) |
| GND            | Pin 33     | Negative terminal of the button (–) |
| UART0 TX       | Pin 1      | UART0 RX (Pin 2)                     |
| SWCLK          | —          | SPI0 SCK (Pin 4)                     |
| GND            | —          | GND                                  |
| SWDIO          | —          | SPI0 TX (Pin 5)                      |

---

### Receiver Pico (RX)

| Function        | Pico Pin   | Connected To                         |
|----------------|------------|--------------------------------------|
| GND            | Pin 3      | GND on LCD1602                       |
| 3V3 OUT        | Pin 36     | VCC on LCD1602                       |
| GPIO4          | Pin 6      | SDA on LCD1602 (I2C0 SDA / UART1 TX)|
| GPIO5          | Pin 7      | SCL on LCD1602 (I2C1 SCL / UART1 RX)|
| GND            | Pin 8      | Negative terminal of the button (–) |
| ADC1           | Pin 32     | Positive terminal of the button (+) |

---

## Development Environment

- Visual Studio Code 2024 Edition
- Windows 11

---

## Directory Structure

Create a top-level directory:

```
C:\VSARM
```

Then create the following subdirectories:

```
C:\VSARM\armcc
C:\VSARM\lib
C:\VSARM\mingw
C:\VSARM\sdk
```

---

## Debugging Setup

1. Clone and build the picoprobe:
   ```
   git clone https://github.com/raspberrypi/picoprobe
   cmake -G "MinGW Makefiles" -DDEBUG_ON_PICO=ON
   make -j4
   ```
   Do this inside `C:\VSARM\sdk`.

2. Download the latest version of OpenOCD.

3. Download and install Zadig:
   - Use it to install the `libusb-win32` driver.
   - Once the firmware is flashed, `picoprobe` should appear in the device list.

---

## Required Software (Rust and C Toolchains)

### Rust Toolchain

- Install the latest version of Cargo
- Install the latest version of `rustc`

---

### C Toolchain

1. Install the GNU ARM Embedded Toolchain  
   Target: AArch32 bare-metal (`arm-none-eabi`)  
   Install to:  
   ```
   C:\VSARM\armcc
   ```

2. Download and extract MinGW-w64 GCC binaries:  
   https://github.com/niXman/mingw-builds-binaries/releases  
   Extract to:  
   ```
   C:\VSARM\mingw
   ```

3. Install or update the following Visual Studio Code extensions:
   - CMake Tools
   - Raspberry Pi Pico SDK

4. Clone the Pico SDK in `C:\VSARM\sdk`:
   ```
   cd /c/VSARM/sdk
   git clone -b master https://github.com/raspberrypi/pico-sdk.git
   cd pico-sdk
   git submodule update --init

   cd ..
   git clone -b master https://github.com/raspberrypi/pico-examples.git
   ```

5. Optionally add toolchain binaries (from the `/bin` directories) to your system PATH variable for command-line access.

---

## Project Builds

### 1. MorseC (Bare-Metal C Implementation)

To Compile:
```
mingw32-make
```
or simply:
```
make
```
The Makefile will auto-generate all required files.

---

### 2. MorseR (Bare-Metal Rust Implementation)

To Compile:
```
cargo build --release
```
or:
```
cargo build
```

After building, navigate to:
```
MorseR\target\thumbv6m-none-eabi\debug\build\MorseR-<build-hash>\out
```

You will find the compiled `transmitter.elf` file in this temporary folder.

To run a debug session:
```
arm-none-eabi-gdb transmitter.elf
```

---

## Final Notes

Ensure all tools are installed in the correct directories under `C:\VSARM`.  
Make sure your system environment variables are correctly configured if you want to invoke toolchains directly via the terminal or command prompt.
