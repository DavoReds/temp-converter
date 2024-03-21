use crate::cli::Unit;
use std::fmt::Display;
use temp_converter_lib::{
    celsius_to_fahrenheit_f32, celsius_to_fahrenheit_f64, celsius_to_kelvin_f32,
    celsius_to_kelvin_f64, fahrenheit_to_celsius_f32, fahrenheit_to_celsius_f64,
    fahrenheit_to_kelvin_f32, fahrenheit_to_kelvin_f64, kelvin_to_celsius_f32,
    kelvin_to_celsius_f64, kelvin_to_fahrenheit_f32, kelvin_to_fahrenheit_f64,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Float {
    F32(f32),
    F64(f64),
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F32(val) => write!(f, "{val}"),
            Self::F64(val) => write!(f, "{val}"),
        }
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl Float {
    pub fn from_str(val: &str, is_double: bool) -> color_eyre::Result<Self> {
        if is_double {
            let value: f64 = val.parse()?;
            Ok(value.into())
        } else {
            let value: f32 = val.parse()?;
            Ok(value.into())
        }
    }

    pub fn convert(&self, from: Unit, to: Unit) -> Self {
        match from {
            Unit::Kelvin => match to {
                Unit::Kelvin => *self,
                Unit::Celsius => match *self {
                    Self::F32(value) => kelvin_to_celsius_f32(value).into(),
                    Self::F64(value) => kelvin_to_celsius_f64(value).into(),
                },
                Unit::Fahrenheit => match *self {
                    Self::F32(value) => kelvin_to_fahrenheit_f32(value).into(),
                    Self::F64(value) => kelvin_to_fahrenheit_f64(value).into(),
                },
            },
            Unit::Celsius => match to {
                Unit::Kelvin => match *self {
                    Self::F32(value) => celsius_to_kelvin_f32(value).into(),
                    Self::F64(value) => celsius_to_kelvin_f64(value).into(),
                },
                Unit::Celsius => *self,
                Unit::Fahrenheit => match *self {
                    Self::F32(value) => celsius_to_fahrenheit_f32(value).into(),
                    Self::F64(value) => celsius_to_fahrenheit_f64(value).into(),
                },
            },
            Unit::Fahrenheit => match to {
                Unit::Celsius => match *self {
                    Self::F32(value) => fahrenheit_to_celsius_f32(value).into(),
                    Self::F64(value) => fahrenheit_to_celsius_f64(value).into(),
                },
                Unit::Kelvin => match *self {
                    Self::F32(value) => fahrenheit_to_kelvin_f32(value).into(),
                    Self::F64(value) => fahrenheit_to_kelvin_f64(value).into(),
                },
                Unit::Fahrenheit => *self,
            },
        }
    }
}
