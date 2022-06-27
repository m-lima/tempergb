//! Convert a [color temperature](https://en.wikipedia.org/wiki/Color_temperature) into RGB
//!
//! This is a rust port of the work by [Tanner Helland](https://tannerhelland.com/2012/09/18/convert-temperature-rgb-algorithm-code.html)
//!
//! # Example
//!
//! ```
//! use tempergb::{rgb_from_temperature, Color};
//! let temperature = 2500;
//! let rgb = tempergb::rgb_from_temperature(temperature);
//! assert_eq!(rgb, (255, 159, 70));
//! assert_eq!(rgb.r(), 255);
//! assert_eq!(rgb.g(), 159);
//! assert_eq!(rgb.b(), 70);
//! ```

#![deny(
    warnings,
    rust_2018_idioms,
    missing_docs,
    clippy::pedantic,
    clippy::missing_const_for_fn
)]

/// Converts a temperature in Kelvin to a RGB triple ([`Color`](struct.Color)).
///
/// **Note:** The input temperature should be in the [1000, 40000] range. Values outside of this
/// range will be truncated.
pub fn rgb_from_temperature(temperature: impl Into<f64>) -> Color {
    let temperature = {
        let temperature: f64 = temperature.into();
        if temperature < 1000.0 {
            1000.0
        } else if temperature > 40000.0 {
            40000.0
        } else {
            temperature
        }
    } / 100.0;

    let r = if temperature <= 66.0 {
        0xff
    } else {
        into_saturated_u8(329.698_727_446 * (temperature - 60.0).powf(-0.133_204_759_2))
    };

    let g = if temperature <= 66.0 {
        into_saturated_u8(99.470_802_586_1 * temperature.ln() - 161.119_568_166_1)
    } else {
        into_saturated_u8(288.122_169_528_3 * (temperature - 60.0).powf(-0.075_514_849_2))
    };

    let b = if temperature >= 66.0 {
        0xff
    } else if temperature <= 19.0 {
        0
    } else {
        into_saturated_u8(138.517_731_223_1 * (temperature - 10.0).ln() - 305.044_792_730_7)
    };

    Color { r, g, b }
}

#[inline]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
// Allow(clippy::cast_sign_loss, clippy::cast_possible_truncation): The bounds have been previously
// checked
fn into_saturated_u8(float: f64) -> u8 {
    if float < 0.0 {
        0
    } else if float > 255.0 {
        255
    } else {
        float as u8
    }
}

/// The RGB value for a given temperature.
///
/// This type as the return type for [`rgb_from_temperature`](fn.rgb_from_temperature) so that the
/// color components are named.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Expose the contents as a tuple of `(red, green, blue)` channels.
    #[must_use]
    pub const fn into_components(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// The `red` component of this color.
    #[must_use]
    pub const fn r(&self) -> u8 {
        self.r
    }

    /// The `green` component of this color.
    #[must_use]
    pub const fn g(&self) -> u8 {
        self.g
    }

    /// The `blue` component of this color.
    #[must_use]
    pub const fn b(&self) -> u8 {
        self.b
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> Self {
        color.into_components()
    }
}

impl PartialEq<(u8, u8, u8)> for Color {
    fn eq(&self, other: &(u8, u8, u8)) -> bool {
        self.r == other.0 && self.g == other.1 && self.b == other.2
    }
}

#[cfg(test)]
mod tests {
    use super::{rgb_from_temperature, Color};

    fn assert_temperature<F: Into<f64>>(temperature: F, r: u8, g: u8, b: u8) {
        assert_eq!(rgb_from_temperature(temperature), Color { r, g, b });
    }

    #[test]
    fn temperature_0() {
        assert_temperature(0, 255, 67, 0);
    }

    #[test]
    fn temperature_1500() {
        assert_temperature(1500, 255, 108, 0);
    }

    #[test]
    fn temperature_2500() {
        assert_temperature(2500, 255, 159, 70);
    }

    #[test]
    fn temperature_5000() {
        assert_temperature(5000, 255, 228, 205);
    }

    #[test]
    fn temperature_6600() {
        assert_temperature(6600, 255, 255, 255);
    }

    #[test]
    fn temperature_10000() {
        assert_temperature(10000, 201, 218, 255);
    }

    #[test]
    fn temperature_15000() {
        assert_temperature(15000, 181, 205, 255);
    }

    #[test]
    fn temperature_40000() {
        assert_temperature(40000, 151, 185, 255);
    }

    #[test]
    fn temperature_60000() {
        assert_temperature(60000, 151, 185, 255);
    }
}
