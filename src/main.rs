#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
extern crate panic_halt;

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

extern crate embedded_hal;
extern crate stm32f30x_hal as hal;

extern crate embedded_graphics;
extern crate ili9341;

use hal::delay::Delay;
use hal::gpio::gpioa::PA9;
use hal::gpio::gpiob::PB6;
use hal::gpio::gpioc::PC7;
use hal::gpio::{Output, PushPull};
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32f30x;

use cortex_m_rt::entry;

use embedded_graphics::fonts::Font12x16;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::fonts::Font8x16;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::RgbColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle, Triangle};

use ili9341::spi::SpiInterface;
use ili9341::*;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(64.mhz())
        .hclk(64.mhz())
        .pclk1(32.mhz())
        .pclk2(32.mhz())
        .freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    // LCD SPI interface
    let sck = gpiob.pb13.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
    let miso = gpiob.pb14.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
    let mosi = gpiob.pb15.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
    let spi = Spi::spi2(
        p.SPI2,
        (sck, miso, mosi),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        },
        32.mhz(),
        clocks,
        &mut rcc.apb1,
    );

    // LCD CS, D/C, RESET pins
    let lcd_cs: PB6<Output<PushPull>> = gpiob
        .pb6
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let lcd_dc: PA9<Output<PushPull>> = gpioa
        .pa9
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let lcd_reset: PC7<Output<PushPull>> = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let iface = SpiInterface::new(spi, lcd_cs, lcd_dc);
    let mut display = Ili9341::new(iface, lcd_reset, &mut delay).unwrap();

    let w = display.width() as i32;
    let h = display.height() as i32;
    display.draw(
        Rectangle::new(Point::new(0, 0), Point::new(w - 1, h - 1)).fill(Some(RgbColor::BLACK)),
    );

    display.draw(
        Triangle::new(Point::new(45, 5), Point::new(5, 75), Point::new(85, 75))
            .stroke(Some(RgbColor::RED))
            .fill(Some(RgbColor::RED))
            .stroke_width(3),
    );

    display.draw(
        Circle::new(Point::new(120, 37), 36)
            .stroke(Some(RgbColor::GREEN))
            .fill(Some(RgbColor::GREEN)),
    );

    display.draw(
        Rectangle::new(Point::new(165, 5), Point::new(165 + 69, 5 + 69))
            .stroke(Some(RgbColor::BLUE))
            .fill(Some(RgbColor::BLUE)),
    );

    display.draw(
        Line::new(Point::new(5, 100), Point::new(100, 100))
            .stroke(Some(RgbColor::GREEN))
            .stroke_width(20),
    );

    display.draw(
        Font6x8::render_str("Hello World!")
            .stroke(Some(RgbColor::RED))
            .stroke_width(2)
            .translate(Point::new(5, 110))
            .into_iter(),
    );

    display.draw(
        Font6x8::render_str("Hello Rust!")
            .stroke(Some(RgbColor::GREEN))
            .stroke_width(2)
            .translate(Point::new(5, 120))
            .into_iter(),
    );

    display.draw(
        Font6x8::render_str("Hello stm32!")
            .stroke(Some(RgbColor::BLUE))
            .stroke_width(2)
            .translate(Point::new(5, 130))
            .into_iter(),
    );

    display.draw(
        Font8x16::render_str("Hello World!")
            .stroke(Some(RgbColor::RED))
            .stroke_width(2)
            .translate(Point::new(5, 150))
            .into_iter(),
    );

    display.draw(
        Font8x16::render_str("Hello Rust!")
            .stroke(Some(RgbColor::GREEN))
            .stroke_width(2)
            .translate(Point::new(5, 170))
            .into_iter(),
    );

    display.draw(
        Font8x16::render_str("Hello stm32!")
            .stroke(Some(RgbColor::BLUE))
            .stroke_width(2)
            .translate(Point::new(5, 190))
            .into_iter(),
    );

    display.draw(
        Font12x16::render_str("Hello World!")
            .stroke(Some(RgbColor::RED))
            .stroke_width(2)
            .translate(Point::new(5, 220))
            .into_iter(),
    );

    display.draw(
        Font12x16::render_str("Hello Rust!")
            .stroke(Some(RgbColor::GREEN))
            .stroke_width(2)
            .translate(Point::new(5, 250))
            .into_iter(),
    );

    display.draw(
        Font12x16::render_str("Hello stm32!")
            .stroke(Some(RgbColor::BLUE))
            .stroke_width(2)
            .translate(Point::new(5, 280))
            .into_iter(),
    );

    loop {}
}
