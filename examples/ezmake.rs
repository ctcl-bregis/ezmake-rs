#![no_std]
#![no_main]

use bsp::hal;
use pyportal as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::DisplaySize240x320;
use bsp::Orientation;
use bsp::{entry, pin_alias};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    let (mut disp, backlight, _tft_te) = pins.display.init(DisplaySize240x320, Orientation::LandscapeFlipped, &mut delay);

    backlight.into_push_pull_output().set_high().unwrap();

    let yoffset = 24;
    let x_max = (disp.width() as i32) - 1;
    let y_max = (disp.height() as i32) - 1;

    let red_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(1)
        .build();

    let green_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::GREEN)
        .stroke_width(1)
        .build();

    let blue_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::BLUE)
        .stroke_width(1)
        .build();

    let gold_style_line = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::new(29u8, 51u8, 0u8))
        .stroke_width(1)
        .build();
    
    let black_fill = PrimitiveStyleBuilder::with_fill

    

    loop {
        delay.delay_ms(400_u16);
        red_led.set_high().unwrap();
        delay.delay_ms(400_u16);
        red_led.set_low().unwrap();
    }
}
