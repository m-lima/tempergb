#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl std::convert::Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

pub fn rgb_from_temperature<F: Into<f64>>(temperature: F) -> Color {
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
        (329.698727446 * (temperature - 60.0).powf(-0.1332047592)) as u8
    };

    let g = if temperature <= 66.0 {
        (99.4708025861 * temperature.ln() - 161.1195681661) as u8
    } else {
        (288.1221695283 * (temperature - 60.0).powf(-0.0755148492)) as u8
    };

    let b = if temperature >= 66.0 {
        0xff
    } else if temperature <= 19.0 {
        0
    } else {
        (138.5177312231 * (temperature - 10.0).ln() - 305.0447927307) as u8
    };

    Color { r, g, b }
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
