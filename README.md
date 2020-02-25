
# `hello-nucleo-oled`
SSD1331 OLED display shown "Hello world".
The device uses a hardware SPI interface to communicate.

# Required hardwares
- stm32nucleo development board 'NUCLEO-F302R8'
- 0.95 inches SSD1331 based Color OLED display module [akizukidenshi.com](http://akizukidenshi.com/) 'P-14435'
- breadboard and wires

# Wiring
| 'NUCLEO-F302R8' arduino connector pins | 'OLED display' pins |
----|----
| GND (CN6)      | GND |
| 3V3 (CN6)      | VCC |
| D13 (CN5) SCK  | SCL |
| D11 (CN5) MOSI | SDA |
| D9  (CN5)      | RES |
| D8  (CN5)      | DC  |
| D10 (CN5) CS   | CS  |

# Build
cargo build --release

# License
Licensed under

Apache License, Version 2.0 (See the 'LICENSE' file).

This project includes software licensed under the Apache License.
