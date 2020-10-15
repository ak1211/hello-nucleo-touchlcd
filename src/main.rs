#![no_std]
#![no_main]

#[allow(unused_extern_crates)]
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use stm32f30x_hal as hal;

use hal::delay::Delay;
use hal::gpio::{gpioa::PA9, gpiob::PB6, gpioc::PC7, Output, PushPull};
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32f30x;

use cortex_m_rt::entry;

use embedded_graphics::fonts::{Font12x16, Font6x8, Font8x16, Text};
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle, Triangle};
use embedded_graphics::style::{PrimitiveStyleBuilder, TextStyle, TextStyleBuilder};

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

    let mut display = Ili9341::new_spi(spi, lcd_cs, lcd_dc, lcd_reset, &mut delay).unwrap();

    display.set_orientation(Orientation::Portrait).unwrap();

    let w = display.width() as i32;
    let h = display.height() as i32;

    Rectangle::new(Point::new(0, 0), Point::new(w - 1, h - 1))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Triangle::new(Point::new(45, 5), Point::new(5, 75), Point::new(85, 75))
        .into_styled(PrimitiveStyleBuilder::new().fill_color(Rgb565::RED).build())
        .draw(&mut display)
        .unwrap();

    Circle::new(Point::new(120, 37), 36)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::GREEN)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Rectangle::new(Point::new(165, 5), Point::new(165 + 69, 5 + 69))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLUE)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Line::new(Point::new(5, 100), Point::new(100, 100))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::GREEN)
                .stroke_width(2)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Text::new("Hello World!", Point::new(5, 110))
        .into_styled(TextStyle::new(Font6x8, Rgb565::RED))
        .draw(&mut display)
        .unwrap();

    Text::new("Hello Rust!", Point::new(5, 120))
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .text_color(Rgb565::GREEN)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Text::new("Hello stm32!", Point::new(5, 130))
        .into_styled(TextStyle::new(Font6x8, Rgb565::BLUE))
        .draw(&mut display)
        .unwrap();

    Text::new("Hello World!", Point::new(5, 150))
        .into_styled(TextStyle::new(Font8x16, Rgb565::RED))
        .draw(&mut display)
        .unwrap();

    Text::new("Hello Rust!", Point::new(5, 170))
        .into_styled(TextStyle::new(Font8x16, Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    Text::new("Hello stm32!", Point::new(5, 190))
        .into_styled(TextStyle::new(Font8x16, Rgb565::BLUE))
        .draw(&mut display)
        .unwrap();

    Text::new("Hello World!", Point::new(5, 220))
        .into_styled(TextStyle::new(Font12x16, Rgb565::RED))
        .draw(&mut display)
        .unwrap();

    Text::new("Hello Rust!", Point::new(5, 250))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    Text::new("Hello stm32!", Point::new(5, 280))
        .into_styled(TextStyle::new(Font12x16, Rgb565::BLUE))
        .draw(&mut display)
        .unwrap();

    loop {}
}
