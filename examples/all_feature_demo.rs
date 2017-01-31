#[macro_use]
extern crate error_chain;
extern crate sysfs_led;

use std::thread;
use std::time::Duration;
use sysfs_led::{Brightness, Led, RgbLed, SysfsLed, SysfsRgbLed};
use sysfs_led::colors::Color;
use sysfs_led::errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let mut red = SysfsLed::new("redLed")?;
    let mut green = SysfsLed::new("grnLed")?;
    let mut blue = SysfsLed::new("bluLed")?;

    println!("Clear LEDs");
    red.set_brightness(Brightness::Off)?;
    green.set_brightness(Brightness::Off)?;
    blue.set_brightness(Brightness::Off)?;

    println!("Red LED on");
    red.set_brightness(Brightness::Full)?;
    delay();
    red.set_brightness(Brightness::Off)?;

    println!("Green LED on");
    green.set_brightness(Brightness::Full)?;
    delay();
    green.set_brightness(Brightness::Off)?;

    println!("Blue LED on");
    blue.set_brightness(Brightness::Full)?;
    delay();
    blue.set_brightness(Brightness::Off)?;

    println!("Flash Red LED");

    // let mut led = SysfsRgbLed::new("redLed", "grnLed", "bluLed").unwrap();
    // loop {
    //     for i in 0..255u8 {
    //         led.set_color(Color::from_hsl(i, 128, 32)).unwrap();
    //     }
    // }

    Ok(())
}

fn delay() {
    thread::sleep(Duration::new(3, 0));
}
