[![CI status](https://gitlab.com/DavoReds/temp-converter/badges/main/pipeline.svg)](https://gitlab.com/DavoReds/tasker/-/commits/main)

# Temperature Converter

Simple CLI utility to convert temperature units between Celsius, Fahrenheit and Kelvin. Can be used with either `f32` or `f64` types.

> [!IMPORTANT] If you want the library
> This crate used to have both a binary and a library version, but now they've separated into two different crates in a workspace. This way, you don't have to import any of the CLI program's dependencies if you only want to convert units yourself. You should instead point to [temp-converter-lib](https://crates.io/crates/temp-converter-lib). Heads up, the interface has been changed and might break your program.
