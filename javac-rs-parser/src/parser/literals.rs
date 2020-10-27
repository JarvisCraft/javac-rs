use num_traits::Num;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

pub trait PrimitiveFrom<T> {
    fn from(source: T) -> Self;
}

impl PrimitiveFrom<u32> for i32 {
    fn from(source: u32) -> Self {
        source as Self
    }
}

impl PrimitiveFrom<u64> for i64 {
    fn from(source: u64) -> Self {
        source as Self
    }
}

pub trait IntoStaticStr {
    fn into_static_str(self) -> &'static str;
}

impl IntoStaticStr for ParseIntError {
    fn into_static_str(self) -> &'static str {
        "integer literal cannot be parsed"
    }
}

impl IntoStaticStr for ParseFloatError {
    fn into_static_str(self) -> &'static str {
        "floating point literal cannot be parsed"
    }
}

pub fn parse_integer_number<F: Num, T: Num + PrimitiveFrom<F>>(
    digits: &str,
    radix: u32,
) -> Result<T, &'static str>
where
    <F as Num>::FromStrRadixErr: IntoStaticStr,
{
    F::from_str_radix(digits.replace('_', "").as_str(), radix)
        .map(|value| T::from(value))
        .map_err(|error| error.into_static_str())
}

pub fn parse_number_i32(digits: &str, radix: u32) -> Result<i32, &'static str> {
    parse_integer_number::<u32, i32>(digits, radix)
}

pub fn parse_number_i64(digits: &str, radix: u32) -> Result<i64, &'static str> {
    parse_integer_number::<u64, i64>(digits, radix)
}

pub fn parse_floating_point_number<T: FromStr>(digits: &str) -> Result<T, &'static str>
where
    T::Err: IntoStaticStr,
{
    digits
        .replace('_', "")
        .parse::<T>()
        .map_err(|error| error.into_static_str())
}

pub fn parse_from_parts<T: FromStr>(
    integer_digits: Option<&str>,
    decimal_digits: Option<&str>,
    exponent_digits: Option<&str>,
    radix: u32,
) -> Result<T, &'static str>
where
    T::Err: IntoStaticStr,
{
    format!(
        "{}.{}e{}",
        integer_digits
            .map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))
            .map_err(|error| error.into_static_str())?,
        decimal_digits
            .map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))
            .map_err(|error| error.into_static_str())?,
        exponent_digits
            .map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))
            .map_err(|error| error.into_static_str())?
    )
    .parse::<T>()
    .map_err(|error| error.into_static_str())
}
