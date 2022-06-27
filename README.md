# tempergb

[![Github](https://github.com/m-lima/tempergb/workflows/build/badge.svg)](https://github.com/m-lima/tempergb/actions?workflow=build)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Cargo](https://img.shields.io/crates/v/tempergb.svg)](https://crates.io/crates/tempergb)
[![Documentation](https://docs.rs/tempergb/badge.svg)](https://docs.rs/tempergb)

Convert a [color temperature](https://en.wikipedia.org/wiki/Color_temperature) into RGB

This is a rust port of the work by [Tanner Helland](https://tannerhelland.com/2012/09/18/convert-temperature-rgb-algorithm-code.html)

## Example

```rust
use tempergb::{rgb_from_temperature, Color};
let temperature = 2500;
let rgb = tempergb::rgb_from_temperature(temperature);
assert_eq!(rgb, (255, 159, 70));
assert_eq!(rgb.r(), 255);
assert_eq!(rgb.g(), 159);
assert_eq!(rgb.b(), 70);
```
