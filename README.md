# `hello-nucleo-touchlcd`
TFT LCD shown "Hello world".  
The device uses a hardware SPI interface to communicate.

# Required hardwares
- stm32nucleo development board 'NUCLEO-F302R8'
- 2.8 inches ILI9341 based Color TFT LCD module [amazon.com](https://www.amazon.co.jp//dp/B072N551V3)
- breadboard and wires

# Wiring
| 'NUCLEO-F302R8' arduino connector pins | 'TFT LCD' pins |
----|----
| 5V (CN6)       | VCC |
| GND (CN6)      | GND |
| D10 (CN5) CS   | CS  |
| D9 (CN5)       | RESET |
| D8 (CN5)       | DC |
| D11 (CN5) MOSI | SDI(MOSI) |
| D13 (CN5) SCK  | SCK  |
| 5V             | LED  |
| D12 (CN5) MISO | SDO(MISO) |

![wiring](https://user-images.githubusercontent.com/17291748/75626443-ff3f7a80-5c0a-11ea-8253-deafa481ee42.JPG)
![Hello, World](https://user-images.githubusercontent.com/17291748/75626469-2b5afb80-5c0b-11ea-8b0f-1cfe713f0244.JPG)

# Build
$ cargo build --release
$ cargo objcopy --release -- -O binary firmware.bin

# License
Licensed under

Apache License, Version 2.0 (See the 'LICENSE' file).

This project includes software licensed under the Apache License.
