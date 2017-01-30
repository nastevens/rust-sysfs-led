// Copyright (c) 2017 Nick Stevens <nick@bitcurry.com>

//! Colorspace tools for RGB LEDs

use std::cmp;

/// RGB Black
pub const BLACK: Color = Color(0, 0, 0);
/// RGB White
pub const WHITE: Color = Color(255, 255, 255);
/// RGB Red
pub const RED: Color = Color(255, 0, 0);
/// RGB Green
pub const GREEN: Color = Color(0, 255, 0);
/// RGB Blue
pub const BLUE: Color = Color(0, 0, 255);
/// RGB Yellow
pub const YELLOW: Color = Color(255, 255, 0);
/// RGB Cyan
pub const CYAN: Color = Color(0, 255, 255);
/// RGB Magenta
pub const MAGENTA: Color = Color(255, 0, 255);

/// Representation of color in RGB colorspace
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Create a new `Color` from red, green, and blue components
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color(red, green, blue)
    }

    /// Create a new `Color` from hue, saturation, and value components.
    ///
    /// Create a `Color` from HSV. Hue is the angle on a circle, with 0 equal
    /// to 0 degrees and 255 equal to 360 degrees. Saturation and Value are
    /// percents, with 0 equal to 0%, and 255 equal to 100%.
    pub fn from_hsv(hue: u8, saturation: u8, value: u8) -> Color {
        if saturation == 0 {
            // color is greyscale
            return Color(value, value, value);
        }

        // make hue 0-5
        let region = hue / 43;
        // find remainder part, make it from 0-255
        let fpart = (hue % 43) * 6;

        // calculate temp vars, doing integer multiplication
        let f = fpart as u16;
        let v = value as u16;
        let s = saturation as u16;
        let p = ((v * (255 - s)) >> 8) as u8;
        let q = ((v * (255 - ((s * f) >> 8))) >> 8) as u8;
        let t = ((v * (255 - ((s * (255 - f)) >> 8))) >> 8) as u8;

        match region {
            0 => Color(value, t, p),
            1 => Color(q, value, p),
            2 => Color(p, value, t),
            3 => Color(p, q, value),
            4 => Color(t, p, value),
            _ => Color(value, p, q),
        }
    }

    /// Create a new `Color` from hue, saturation, and lightness components.
    ///
    /// Create a `Color` from HSL. Hue is the angle on a circle, with 0 equal
    /// to 0 degrees and 255 equal to 360 degrees. Saturation and Lightness are
    /// percents, with 0 equal to 0%, and 255 equal to 100%.
    pub fn from_hsl(hue: u8, saturation: u8, lightness: u8) -> Color {
        if saturation == 0 {
            // color is greyscale
            return Color(lightness, lightness, lightness);
        }

        // make hue 0-5
        let region = hue / 43;
        // find remainder part, make it from 0-255
        let fpart = (hue % 43) * 6;

        // calculate temp vars, doing integer multiplication
        let f = fpart as u16;
        let l = lightness as u16;
        let s = saturation as u16;

        let chroma = if saturation < 128 {
            s.saturating_mul(l) >> 7
        } else {
            s.saturating_mul(255 - l) >> 7
        };

        let m = l.saturating_sub(chroma >> 1) as u8;
        let c = (chroma as u8).saturating_add(m);
        let x1 = ((chroma.saturating_mul(f) >> 8) as u8).saturating_add(m);
        let x2 = ((chroma.saturating_mul(255 - f) >> 8) as u8).saturating_add(m);

        match region {
            0 => Color(c, x1, m),
            1 => Color(x2, c, m),
            2 => Color(m, c, x1),
            3 => Color(m, x2, c),
            4 => Color(x1, m, c),
            _ => Color(c, m, x2),
        }
    }

    // pub fn to_hsl(&self) -> (u8, u8, u8) {
    //     let red = self.red() as u16;
    //     let green = self.green() as u16;
    //     let blue = self.blue() as u16;

    //     let cmax = cmp::max(cmp::max(red, green), blue);
    //     let cmin = cmp::min(cmp::min(red, green), blue);
    //     let delta = cmax - cmin;

    //     let hue = if delta == 0 {
    //         0
    //     } else if cmax == red {
    //         43 * (green - blue) / delta
    //         43 * (((self.green() - self.blue()) / delta) % 6)
    //     } else if cmax == self.green() {
    //         43 * (((self.blue() - self.red()) / delta ) + 2)
    //     } else {
    //         43 * (((self.red() - self.green()) / delta) + 4)
    //     };

    //     let lightness = (cmax + cmin) >> 1;

    //     let saturation = if delta == 0 {
    //         0
    //     } else if lightness < 128 {
    //         delta / (255 - (lightness << 1))
    //     } else {
    //         delta / ((lightness << 1) - 255)
    //     };

    //     (hue, saturation, lightness)
    // }

    pub fn red(&self) -> u8 {
        self.0
    }

    pub fn green(&self) -> u8 {
        self.1
    }

    pub fn blue(&self) -> u8 {
        self.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(Color(0, 0, 0), Color::from_hsv(0, 0, 0));
        assert_eq!(Color(255, 255, 255), Color::from_hsv(0, 0, 255));
        assert_eq!(Color(255, 0, 0), Color::from_hsv(0, 255, 255));
        assert_eq!(Color(0, 255, 0), Color::from_hsv(86, 255, 255));
        assert_eq!(Color(0, 0, 255), Color::from_hsv(172, 255, 255));
        assert_eq!(Color(254, 255, 0), Color::from_hsv(43, 255, 255));
        assert_eq!(Color(0, 254, 255), Color::from_hsv(129, 255, 255));
        assert_eq!(Color(255, 0, 254), Color::from_hsv(215, 255, 255));
        assert_eq!(Color(192, 192, 192), Color::from_hsv(0, 0, 192));
        assert_eq!(Color(128, 128, 128), Color::from_hsv(0, 0, 128));
        assert_eq!(Color(128, 0, 0), Color::from_hsv(0, 255, 128));
        assert_eq!(Color(127, 128, 0), Color::from_hsv(43, 255, 128));
        assert_eq!(Color(0, 128, 0), Color::from_hsv(86, 255, 128));
        assert_eq!(Color(128, 0, 127), Color::from_hsv(215, 255, 128));
        assert_eq!(Color(0, 128, 126), Color::from_hsv(128, 255, 128));
        assert_eq!(Color(0, 0, 128), Color::from_hsv(172, 255, 128));
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_eq!(Color(  0,   0,   0), Color::from_hsl(  0,   0,   0));
        assert_eq!(Color(255, 255, 255), Color::from_hsl(  0,   0, 255));
        assert_eq!(Color(255, 255, 255), Color::from_hsl(255,   0, 255));
        assert_eq!(Color(255, 255, 255), Color::from_hsl(255, 255, 255));
        assert_eq!(Color(127, 127, 127), Color::from_hsl(  0,   0, 127));
        assert_eq!(Color(255,   0,   0), Color::from_hsl(  0, 255, 127));
        assert_eq!(Color(255, 125,   0), Color::from_hsl( 21, 255, 127));
        assert_eq!(Color(254, 255,   0), Color::from_hsl( 43, 255, 127));
        assert_eq!(Color(128, 255,   0), Color::from_hsl( 64, 255, 127));
        assert_eq!(Color(  0, 255, 251), Color::from_hsl(128, 255, 127));
        assert_eq!(Color(125,   0, 255), Color::from_hsl(193, 255, 127));
        assert_eq!(Color(190, 126,  64), Color::from_hsl( 21, 127, 127));
        assert_eq!(Color(189, 190,  64), Color::from_hsl( 43, 127, 127));
        assert_eq!(Color(127, 190,  64), Color::from_hsl( 64, 127, 127));
        assert_eq!(Color( 64, 190, 188), Color::from_hsl(128, 127, 127));
        assert_eq!(Color(126,  64, 190), Color::from_hsl(193, 127, 127));
    }
}
