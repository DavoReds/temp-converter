/// Takes an `f32` value representing a Kelvin unit and returns its equivalent
/// in Celsius.
#[must_use]
pub fn kelvin_to_celsius_f32(val: f32) -> f32 {
    val - 273.15
}

/// Takes an `f64` value representing a Kelvin unit and returns its equivalent
/// in Celsius.
#[must_use]
pub fn kelvin_to_celsius_f64(val: f64) -> f64 {
    val - 273.15
}

/// Takes an `f32` value representing a Kelvin unit and returns its equivalent
/// in Fahrenheit.
#[must_use]
pub fn kelvin_to_fahrenheit_f32(val: f32) -> f32 {
    (val - 273.15).mul_add(9_f32 / 5_f32, 32_f32)
}

/// Takes an `f64` value representing a Kelvin unit and returns its equivalent
/// in Fahrenheit.
#[must_use]
pub fn kelvin_to_fahrenheit_f64(val: f64) -> f64 {
    (val - 273.15).mul_add(9_f64 / 5_f64, 32_f64)
}

/// Takes an `f32` value representing a Celsius unit and returns its equivalent
/// in Kelvin.
#[must_use]
pub fn celsius_to_kelvin_f32(val: f32) -> f32 {
    val + 273.15
}

/// Takes an `f64` value representing a Celsius unit and returns its equivalent
/// in Kelvin.
#[must_use]
pub fn celsius_to_kelvin_f64(val: f64) -> f64 {
    val + 273.15
}

/// Takes an `f32` value representing a Celsius unit and returns its equivalent
/// in Fahrenheit.
#[must_use]
pub fn celsius_to_fahrenheit_f32(val: f32) -> f32 {
    val.mul_add(9_f32 / 5_f32, 32_f32)
}

/// Takes an `f64` value representing a Celsius unit and returns its equivalent
/// in Fahrenheit.
#[must_use]
pub fn celsius_to_fahrenheit_f64(val: f64) -> f64 {
    val.mul_add(9_f64 / 5_f64, 32_f64)
}

/// Takes an `f32` value representing a Fahrenheit unit and returns its
/// equivalent in Kelvin.
#[must_use]
pub fn fahrenheit_to_kelvin_f32(val: f32) -> f32 {
    (val - 32_f32).mul_add(5_f32 / 9_f32, 273.15)
}

/// Takes an `f64` value representing a Fahrenheit unit and returns its
/// equivalent in Kelvin.
#[must_use]
pub fn fahrenheit_to_kelvin_f64(val: f64) -> f64 {
    (val - 32_f64).mul_add(5_f64 / 9_f64, 273.15)
}

/// Takes an `f32` value representing a Fahrenheit unit and returns its
/// equivalent in Celsius.
#[must_use]
pub fn fahrenheit_to_celsius_f32(val: f32) -> f32 {
    (val - 32_f32) * (5_f32 / 9_f32)
}

/// Takes an `f64` value representing a Fahrenheit unit and returns its
/// equivalent in Celsius.
#[must_use]
pub fn fahrenheit_to_celsius_f64(val: f64) -> f64 {
    (val - 32_f64) * (5_f64 / 9_f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// Checks whether the difference between two `f32` values is less than EPSILON
    fn float_equals_f32(x: f32, y: f32) -> bool {
        (x - y).abs() < f32::EPSILON
    }

    #[rstest]
    #[case(0.0, -273.15)]
    #[case(100.0, -173.15)]
    #[case(1000.0, 726.85)]
    #[case(5000.0, 4726.85)]
    #[case(5000.0, 4726.85)]
    fn kelvin_to_celsius_f32_works(#[case] input: f32, #[case] expected: f32) {
        let result = kelvin_to_celsius_f32(input);
        assert!(
            float_equals_f32(result, expected),
            "Expected: {expected}\tReceived: {result}"
        );
    }

    #[rstest]
    #[case(0.0, -459.66998)]
    #[case(100.0, -279.66998)]
    #[case(1000.0, 1340.33)]
    #[case(5000.0, 8540.33)]
    #[case(69_420.0, 124_496.33)]
    fn kelvin_to_fahrenheit_f32_works(#[case] input: f32, #[case] expected: f32) {
        let result = kelvin_to_fahrenheit_f32(input);
        assert!(
            float_equals_f32(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, 273.15)]
    #[case(100.0, 373.15)]
    #[case(1000.0, 1273.15)]
    #[case(5000.0, 5273.15)]
    #[case(69_420.0, 69_693.15)]
    fn celsius_to_kelvin_f32_works(#[case] input: f32, #[case] expected: f32) {
        let result = celsius_to_kelvin_f32(input);
        assert!(
            float_equals_f32(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, 32.0)]
    #[case(100.0, 212.0)]
    #[case(1000.0, 1832.0)]
    #[case(5000.0, 9032.0)]
    #[case(69_420.0, 124_988.0)]
    fn celsius_to_fahrenheit_f32_works(#[case] input: f32, #[case] expected: f32) {
        let result = celsius_to_fahrenheit_f32(input);
        assert!(
            float_equals_f32(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, 255.37222)]
    #[case(100.0, 310.92776)]
    #[case(1000.0, 810.9278)]
    #[case(5000.0, 3033.1501)]
    #[case(69_420.0, 38_822.04)]
    fn fahreheit_to_kelvin_f32_works(#[case] input: f32, #[case] expected: f32) {
        let result = fahrenheit_to_kelvin_f32(input);
        assert!(
            float_equals_f32(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, -17.777_779)]
    #[case(100.0, 37.77778)]
    #[case(1000.0, 537.77783)]
    #[case(5000.0, 2_760.000_2)]
    #[case(69_420.0, 38_548.89)]
    fn fahreheit_to_celsius_f32_works(#[case] input: f32, #[case] expected: f32) {
        let result = fahrenheit_to_celsius_f32(input);
        assert!(
            float_equals_f32(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    /// Checks whether the difference between two `f64` values is less than EPSILON
    fn float_equals_f64(x: f64, y: f64) -> bool {
        (x - y).abs() < f64::EPSILON
    }

    #[rstest]
    #[case(0.0, -273.15)]
    #[case(100.0, -173.149_999_999_999_98)]
    #[case(1000.0, 726.85)]
    #[case(5000.0, 4726.85)]
    #[case(5000.0, 4726.85)]
    fn kelvin_to_celsius_f64_works(#[case] input: f64, #[case] expected: f64) {
        let result = kelvin_to_celsius_f64(input);
        assert!(
            float_equals_f64(result, expected),
            "Expected: {expected}\tReceived: {result}",
        );
    }

    #[rstest]
    #[case(0.0, -459.669_999_999_999_96)]
    #[case(100.0, -279.669_999_999_999_96)]
    #[case(1000.0, 1_340.330_000_000_000_2)]
    #[case(5000.0, 8_540.330_000_000_002)]
    #[case(69_420.0, 124_496.330_000_000_02)]
    fn kelvin_to_fahrenheit_f64_works(#[case] input: f64, #[case] expected: f64) {
        let result = kelvin_to_fahrenheit_f64(input);
        assert!(
            float_equals_f64(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, 273.15)]
    #[case(100.0, 373.15)]
    #[case(1000.0, 1273.15)]
    #[case(5000.0, 5273.15)]
    #[case(69_420.0, 69_693.15)]
    fn celsius_to_kelvin_f64_works(#[case] input: f64, #[case] expected: f64) {
        let result = celsius_to_kelvin_f64(input);
        assert!(
            float_equals_f64(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, 32.0)]
    #[case(100.0, 212.0)]
    #[case(1000.0, 1832.0)]
    #[case(5000.0, 9032.0)]
    #[case(69_420.0, 124_988.0)]
    fn celsius_to_fahrenheit_f64_works(#[case] input: f64, #[case] expected: f64) {
        let result = celsius_to_fahrenheit_f64(input);
        assert!(
            float_equals_f64(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, 255.372_222_222_222_2)]
    #[case(100.0, 310.927_777_777_777_75)]
    #[case(1000.0, 810.927_777_777_777_8)]
    #[case(5000.0, 3033.15)]
    #[case(69_420.0, 38_822.038_888_888_89)]
    fn fahreheit_to_kelvin_f64_works(#[case] input: f64, #[case] expected: f64) {
        let result = fahrenheit_to_kelvin_f64(input);
        assert!(
            float_equals_f64(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }

    #[rstest]
    #[case(0.0, -17.777_777_777_777_78)]
    #[case(100.0, 37.777_777_777_777_78)]
    #[case(1000.0, 537.777_777_777_777_8)]
    #[case(5000.0, 2760.0)]
    #[case(69_420.0, 38_548.888_888_888_89)]
    fn fahreheit_to_celsius_f64_works(#[case] input: f64, #[case] expected: f64) {
        let result = fahrenheit_to_celsius_f64(input);
        assert!(
            float_equals_f64(result, expected),
            "Expected: {expected}\t Received: {result}",
        );
    }
}
