# Temperature Converter

This is a no dependency library to convert between Kelvin, Celsius and
Fahrenheit units. Because Rust has no concept of type unions, each conversion
function has two versions: `_f32` and `_f64`.
