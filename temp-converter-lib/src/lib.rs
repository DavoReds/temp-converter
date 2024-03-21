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

    /// Checks whether the difference between two `f32` values is less than EPSILON
    fn float_equals(x: f32, y: f32) -> bool {
        (x - y).abs() < f32::EPSILON
    }

    #[test]
    fn kelvin_to_celsius_works() {
        let test_cases: [[f32; 2]; 5] = [
            [0.0, -273.15],
            [100.0, -173.15],
            [1000.0, 726.85],
            [5000.0, 4726.85],
            [69_420.0, 69_146.85],
        ];

        for case in test_cases {
            let result = kelvin_to_celsius_f32(case[0]);
            assert!(
                float_equals(result, case[1]),
                "Expected: {}\t Received: {}",
                case[1],
                result
            );
        }
    }

    #[test]
    fn kelvin_to_fahrenheit_works() {
        let test_cases: [[f32; 2]; 5] = [
            [0.0, -459.66998],
            [100.0, -279.66998],
            [1000.0, 1340.33],
            [5000.0, 8540.33],
            [69_420.0, 124_496.33],
        ];

        for case in test_cases {
            let result = kelvin_to_fahrenheit_f32(case[0]);
            assert!(
                float_equals(result, case[1]),
                "Expected: {}\t Received: {}",
                case[1],
                result
            );
        }
    }

    #[test]
    fn celsius_to_kelvin_works() {
        let test_cases: [[f32; 2]; 5] = [
            [0.0, 273.15],
            [100.0, 373.15],
            [1000.0, 1273.15],
            [5000.0, 5273.15],
            [69_420.0, 69_693.15],
        ];

        for case in test_cases {
            let result = celsius_to_kelvin_f32(case[0]);
            assert!(
                float_equals(result, case[1]),
                "Expected: {}\t Received: {}",
                case[1],
                result
            );
        }
    }

    #[test]
    fn celsius_to_fahrenheit_works() {
        let test_cases: [[f32; 2]; 5] = [
            [0.0, 32.0],
            [100.0, 212.0],
            [1000.0, 1832.0],
            [5000.0, 9032.0],
            [69_420.0, 124_988.0],
        ];

        for case in test_cases {
            let result = celsius_to_fahrenheit_f32(case[0]);
            assert!(
                float_equals(result, case[1]),
                "Expected: {}\t Received: {}",
                case[1],
                result
            );
        }
    }

    #[test]
    fn fahreheit_to_kelvin_works() {
        let test_cases: [[f32; 2]; 5] = [
            [0.0, 255.37222],
            [100.0, 310.92776],
            [1000.0, 810.9278],
            [5000.0, 3033.1501],
            [69_420.0, 38_822.04],
        ];

        for case in test_cases {
            let result = fahrenheit_to_kelvin_f32(case[0]);
            assert!(
                float_equals(result, case[1]),
                "Expected: {}\t Received: {}",
                case[1],
                result
            );
        }
    }

    #[test]
    fn fahreheit_to_celsius_works() {
        let test_cases: [[f32; 2]; 5] = [
            [0.0, -17.777_779],
            [100.0, 37.77778],
            [1000.0, 537.77783],
            [5000.0, 2_760.000_2],
            [69_420.0, 38_548.89],
        ];

        for case in test_cases {
            let result = fahrenheit_to_celsius_f32(case[0]);
            assert!(
                float_equals(result, case[1]),
                "Expected: {}\t Received: {}",
                case[1],
                result
            );
        }
    }
}
