// Copyright (c) 2017 Nick Stevens <nick@bitcurry.com>

//! Rust bindings for the [Linux sysfs LED class].
//!
//! Methods and types in this crate center around the [`Led`]
//! and [`RgbLed`] types. Implementations of these types are
//! provided for the Linux sysfs LED API, although an attempt has been made to
//! keep types generic enough that non-sysfs implementions could be possible.
//!
//! In addition to providing safe access to LED types directly, this crate also
//! provides access to a number of `Trigger`s through the [`triggers`] module.
//! Applying a trigger to an [`Led`] or [`RgbLed`] type can introduce a number
//! of automated behaviors, such as [timing],
//! [heartbeat], or [cpu activity].
//!
//! [Linux sysfs LED class]: https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-led
//! [`Led`]: trait.Led.html
//! [`RgbLed`]: trait.RgbLed.html
//! [`triggers`]: triggers/index.html
//! [timing]: triggers/trait.TriggerTimer.html
//! [heartbeat]: triggers/trait.TriggerHeartbeat.html
//! [cpu activity]: triggers/trait.TriggerCpu.html

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[cfg(test)]
extern crate tempdir;

pub mod colors;
pub mod errors;
pub mod triggers;

use std::cmp;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use colors::Color;
use errors::*;

const SYSFS_LED_CLASS: &'static str = "/sys/class/leds";


/// Brightness of an LED
///
/// Output brightness of an LED, always specified against some maximum
/// brightness. Usually this maximum brightness will be 255, but always prefer
/// the `Percent` variant over `Absolute` in case a value other than 255
/// exists.
///
/// ## Note
///
/// The Linux kernel defines the LED brightness as an enum type, so it could be
/// either 32 or 64 bits (or neither, the C spec is murky on this).
/// Realistically, however, it should never be more than 255 because that is
/// the definition for LED_FULL. We use u32 because it makes math easier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Brightness {
    Full,
    Off,
    Percent(u32),
    Absolute(u32),
}

impl Brightness {
    pub fn to_absolute(&self, max_brightness: u32) -> u32 {
        match *self {
            Brightness::Full => max_brightness,
            Brightness::Off => 0,
            Brightness::Percent(p) => max_brightness.saturating_mul(cmp::min(p, 100)) / 100,
            Brightness::Absolute(a) => cmp::min(max_brightness, a),
        }
    }

    pub fn to_percent(&self, max_brightness: u32) -> u32 {
        match *self {
            Brightness::Full => max_brightness,
            Brightness::Off => 0,
            Brightness::Percent(p) => cmp::min(p, 100),
            Brightness::Absolute(a) => {
                cmp::min(a, max_brightness).saturating_mul(100) / max_brightness
            }
        }
    }
}

/// Basic functionality of an LED
///
/// Defines basic functionality of an LED, which is to be turned on or off at
/// some level of brightness.
pub trait Led {
    /// Get the current brightness of an LED
    fn brightness(&self) -> Result<Brightness>;
    /// Set the brightness of an LED
    fn set_brightness(&mut self, brightness: Brightness) -> Result<()>;
}

/// Access to an LED managed by the Linux LED sysfs class driver
pub struct SysfsLed {
    device_path: PathBuf,
}

impl SysfsLed {
    /// Create a new `SysfsLed` with a given name located in the default sysfs
    /// directory
    pub fn new(name: &str) -> Result<SysfsLed> {
        Self::from_path(Path::new(SYSFS_LED_CLASS).join(name))
    }

    /// Create a new `SysfsLed` with a custom path to the sysfs directory for
    /// the LED class device
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<SysfsLed> {
        require_device_files(&path)?;
        Ok(SysfsLed { device_path: path.as_ref().to_path_buf() })
    }

    /// Return the raw max_brightness of the LED device
    pub fn max_brightness(&self) -> Result<u32> {
        Ok(self.sysfs_read_file("max_brightness")?.parse::<u32>()?)
    }

    fn sysfs_read_file(&self, name: &str) -> Result<String> {
        sysfs_read_file(&self.device_path, name)
    }

    fn sysfs_write_file(&self, name: &str, value: &str) -> Result<()> {
        sysfs_write_file(&self.device_path, name, value)
    }
}

impl Led for SysfsLed {
    fn brightness(&self) -> Result<Brightness> {
        Ok(Brightness::Absolute(self.sysfs_read_file("brightness")?.parse::<u32>()?))
    }

    fn set_brightness(&mut self, brightness: Brightness) -> Result<()> {
        let max_brightness = self.max_brightness()?;
        let string_value = format!("{}", brightness.to_absolute(max_brightness));
        self.sysfs_write_file("brightness", &string_value)?;
        Ok(())
    }
}

/// Basic functionality of an LED with red, green, and blue component colors
///
/// By stacking multiple LEDs together, one each of red, blue, and green, it is
/// possible to create a multicolored LED capable of showing many different
/// colors.
pub trait RgbLed: Led {
    /// Get the color of the RGB LED
    fn color(&self) -> Result<Color>;
    /// Set the color of the RGB LED
    fn set_color(&mut self, color: Color) -> Result<()>;
}

/// Access to an RGB LED managed by the Linux LED sysfs class driver,
/// configured as 3 separate LEDs.
pub struct SysfsRgbLed {
    red: SysfsLed,
    green: SysfsLed,
    blue: SysfsLed,
}

impl SysfsRgbLed {
    /// Create a new `SysfsRgbLed` from LEDs with the given names in the
    /// default sysfs directory
    pub fn new(red: &str, green: &str, blue: &str) -> Result<SysfsRgbLed> {
        Self::from_leds(SysfsLed::new(red)?,
                        SysfsLed::new(green)?,
                        SysfsLed::new(blue)?)
    }

    /// Create a new `SysfsRgbLed` with custom paths to the sysfs directories for
    /// the separate LED devices
    pub fn from_path<Pr, Pg, Pb>(red: Pr, green: Pg, blue: Pb) -> Result<SysfsRgbLed>
        where Pr: AsRef<Path>,
              Pg: AsRef<Path>,
              Pb: AsRef<Path>
    {
        Self::from_leds(SysfsLed::from_path(red)?,
                        SysfsLed::from_path(green)?,
                        SysfsLed::from_path(blue)?)
    }

    /// Create a new `SysfsRgbLed` from existing `SysfsLed` objects
    pub fn from_leds(red: SysfsLed, green: SysfsLed, blue: SysfsLed) -> Result<SysfsRgbLed> {
        Ok(SysfsRgbLed {
            red: red,
            green: green,
            blue: blue,
        })
    }
}

impl Led for SysfsRgbLed {
    // Brightness on an RGB LED as a whole is a bit strange since there are
    // three LEDs making up the output. We choose to treat brightness as
    // "lightness" in the HSL color space instead - increasing lightness will
    // increase perceived brightness, so it's close.
    fn brightness(&self) -> Result<Brightness> {
        Ok(Brightness::Off)
    }

    fn set_brightness(&mut self, _brightness: Brightness) -> Result<()> {
        Ok(())
    }
}

impl RgbLed for SysfsRgbLed {
    fn color(&self) -> Result<Color> {
        // TODO: This isn't correct
        let _red = self.red.brightness()?;
        let _green = self.green.brightness()?;
        let _blue = self.blue.brightness()?;
        Ok(Color::from_rgb(0, 0, 0))
    }

    fn set_color(&mut self, color: Color) -> Result<()> {
        let red_max = self.red.max_brightness()? as u32;
        let green_max = self.green.max_brightness()? as u32;
        let blue_max = self.blue.max_brightness()? as u32;
        // TODO: This isn't correct
        self.red.set_brightness(Brightness::Absolute(color.red() as u32))?;
        self.green.set_brightness(Brightness::Absolute(color.green() as u32))?;
        self.blue.set_brightness(Brightness::Absolute(color.blue() as u32))?;
        Ok(())
    }
}

// Make sure that the specified files exist in the given directory
fn require_device_files<D>(dir: D) -> Result<()>
    where D: AsRef<Path>
{
    for file in &["brightness", "max_brightness", "trigger"] {
        if !dir.as_ref().join(file).is_file() {
            bail!(ErrorKind::InvalidDevicePath(dir.as_ref().to_string_lossy().into()));
        }
    }

    Ok(())
}

fn sysfs_read_file(device_path: &Path, name: &str) -> Result<String> {
    let path = device_path.join(name);
    let mut file = OpenOptions::new().read(true)
        .open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result.trim().into())
}

fn sysfs_write_file(device_path: &Path, name: &str, value: &str) -> Result<()> {
    let path = device_path.join(name);
    let mut file = OpenOptions::new().write(true)
        .truncate(true)
        .create(false)
        .open(path)?;
    Ok(file.write_all(value.as_bytes())?)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;

    use tempdir::TempDir;

    use super::*;

    struct SysfsWrapper(TempDir);

    impl SysfsWrapper {
        fn path(&self) -> &Path {
            self.0.path()
        }

        fn get(&self, name: &str) -> String {
            let mut result = String::new();
            File::open(self.path().join(name))
                .expect(&format!("opening {}", name))
                .read_to_string(&mut result)
                .expect(&format!("reading {}", name));
            result
        }

        fn set(&mut self, name: &str, value: &str) {
            File::open(self.path().join(name))
                .expect(&format!("opening {}", name))
                .write_all(value.as_bytes())
                .expect(&format!("writing {}", name));
        }
    }

    macro_rules! create_sysfs_dir {
        ( $name:expr; $( $file:expr => $value:expr );+ ) => {{
            let tempdir = TempDir::new($name).expect("create temp dir");
            $({
                let mut file = File::create(tempdir.path().join($file))
                    .expect(concat!("create ", $file, " file"));
                file.write_all($value.as_bytes())
                    .expect(concat!("writing ", $file, " initial value"));
            })+

            SysfsWrapper(tempdir)
        }};
    }

    #[test]
    fn test_set_brightness() {
        let harness = create_sysfs_dir!("sysfs_led_test";
                                        "brightness" => "0";
                                        "max_brightness" => "128";
                                        "trigger" => "[none]");
        let mut led = SysfsLed::from_path(harness.path()).expect("create sysfs led");
        let vectors = vec![(Brightness::Full, "128"),
                           (Brightness::Percent(50), "64"),
                           (Brightness::Percent(150), "128"),
                           (Brightness::Absolute(0), "0"),
                           (Brightness::Absolute(72), "72"),
                           (Brightness::Absolute(129), "128"),
                           (Brightness::Off, "0")];
        for (brightness, expected) in vectors {
            led.set_brightness(brightness).expect(&format!("setting brightness={:?}", brightness));
            assert_eq!(expected, harness.get("brightness"));
        }
    }
}
