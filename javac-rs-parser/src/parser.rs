pub use javac_rs_ast::ast;
pub use peg;
use std::error::Error;
use std::fmt::Formatter;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    description: String,
}

impl ParseError {
    pub fn new(description: String) -> Self {
        ParseError { description }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Parse error: {}", self.description)
    }
}

impl Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        ParseError::new(error.to_string())
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(error: ParseFloatError) -> Self {
        ParseError::new(error.to_string())
    }
}

fn parse_number_i32(digits: &str, radix: u32) -> Result<i32, ParseError> {
    u32::from_str_radix(digits.replace('_', "").as_str(), radix)
        .map(|value| value as i32)
        .map_err(|error| error.into())
}

fn parse_number_i64(digits: &str, radix: u32) -> Result<i64, ParseError> {
    u64::from_str_radix(digits.replace('_', "").as_str(), radix)
        .map(|value| value as i64)
        .map_err(|error| error.into())
}

fn parse_number_f32(digits: &str) -> Result<f32, ParseError> {
    digits
        .replace('_', "")
        .parse::<f32>()
        .map_err(|error| error.into())
}

fn parse_number_f64(digits: &str) -> Result<f64, ParseError> {
    digits
        .replace('_', "")
        .parse::<f64>()
        .map_err(|error| error.into())
}

fn parse_f32_from_parts(
    integer_digits: Option<&str>,
    decimal_digits: Option<&str>,
    exponent_digits: Option<&str>,
    radix: u32,
) -> Result<f32, ParseError> {
    format!(
        "{}.{}e{}",
        integer_digits.map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))?,
        decimal_digits.map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))?,
        exponent_digits.map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))?
    )
    .parse::<f32>()
    .map_err(|error| error.into())
}

fn parse_f64_from_parts(
    integer_digits: Option<&str>,
    decimal_digits: Option<&str>,
    exponent_digits: Option<&str>,
    radix: u32,
) -> Result<f64, ParseError> {
    format!(
        "{}.{}e{}",
        integer_digits.map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))?,
        decimal_digits.map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))?,
        exponent_digits.map_or(Ok(0), |digits| i64::from_str_radix(digits, radix))?
    )
    .parse::<f64>()
    .map_err(|error| error.into())
}

// TODO remove unneeded pub's
peg::parser! {
    /// Java language grammar as specified by [JLS](https://docs.oracle.com/javase/specs/)
    pub grammar java() for str {

        // Basic lexical objects not bound to AST-nodes

        /// Digit of a binary number
        pub rule binary_digit() -> char
                = slice:$(['0' | '1']) { slice.chars().next().unwrap() }

        /// Digit of an octal number
        pub rule octal_digit() -> char
                = slice:$(['0'..='7']) { slice.chars().next().unwrap() }

        /// Digit of a decimal number
        pub rule decimal_digit() -> char
                = slice:$(['0'..='9']) { slice.chars().next().unwrap() }

        /// Digit of a hex number
        pub rule hex_digit() -> char
                = slice:$(['0'..='9' | 'A'..='F' | 'a'..='f']) { slice.chars().next().unwrap() }

        /// Prefix of a binary number
        pub rule binary_number_prefix() = "0" ['B' | 'b']

        /// Prefix of an octal number
        pub rule octal_number_prefix() = "0"

        /// Prefix of a hex number
        pub rule hex_number_prefix() = "0" ['X' | 'x']

        /// Suffix of `long` number
        pub rule long_number_suffix() = ['L' | 'l']

        /// Suffix of `float` number
        pub rule float_number_suffix() = ['F' | 'f']

        /// Suffix of `double` number
        pub rule double_number_suffix() = ['D' | 'd']

        /// Indicator of a start of the exponent of a decimal number
        pub rule decimal_exponent_indicator() = ['E' | 'e']

        /// Indicator of a start of the exponent of a hex number
        pub rule hex_exponent_indicator() = ['P' | 'p']

        /// Rule returning `-1i32` for `"-"` and `+1i32` for anything other
        pub rule sign_num_i32() -> i32 = "-" { -1 } / { 1 }

        /// Rule returning `-1i64` for `"-"` and `+1i64` for anything other
        pub rule sign_num_i64() -> i64 = "-" { -1 } / { 1 }

        /// Optional separator of digits in numbers
        pub rule digit_separator() = "_"

        /// Separator of integer and fractional parts of numbers
        pub rule decimal_separator() = "."

        /// A character which does not require escaping in char literals
        pub rule not_escaped_char_symbol() -> char
            = character:$(!['\\' | '\''][_]) { character.chars().next().unwrap() };

        pub rule escape_sequence() -> ast::Char
            = "b" { 0x0008 } / "t" { 0x0009 } / "n" { 0x000a } / "f" { 0x000c }
            / "r" { 0x000d } / "\"" { 0x0022 } / "'" { 0x0027 } / "\\" { 0x005c }

        /// Sequence of specified digits with optional
        /// non-trailing [digit separators](digit_separator).
        ///
        /// # Arguments
        ///
        /// * `digit` - rule matching valid number digits
        rule number(digit: rule<char>) -> &'input str = $(digit() (digit_separator()* digit()+)*)

        /// [Hex](hex_digit) [number](number)
        rule hex_number() -> &'input str = number(<hex_digit()>)

        /// [Decimal](decimal_digit) [number](number)
        rule decimal_number() -> &'input str = number(<decimal_digit()>)

        /// [Octal](octal_digit) [number](number)
        rule octal_number() -> &'input str = number(<octal_digit()>)

        /// [Binary](binary_digit) [number](number)
        rule binary_number() -> &'input str = number(<binary_digit()>)

        /// Number with an optional sign
        rule signed_number() -> &'input str = $(['+' | '-']? decimal_number())

        /// Number of type `int`
        pub rule int_number() -> Result<i32, ParseError> = number:(
            (hex_number_prefix() digits:hex_number() { parse_number_i32(digits, 16) })
            / (binary_number_prefix() digits:binary_number() { parse_number_i32(digits, 2) })
            / (octal_number_prefix() digits:octal_number() { parse_number_i32(digits, 8) })
            / (digits:decimal_number() { parse_number_i32(digits, 10)})
        )

        /// Number of type `long`
        pub rule long_number() -> Result<i64, ParseError> = number:(
            (hex_number_prefix() digits:hex_number() { parse_number_i64(digits, 16) })
            / (binary_number_prefix() digits:binary_number() { parse_number_i64(digits, 2) })
            / (octal_number_prefix() digits:octal_number() { parse_number_i64(digits, 8) })
            / (digits:decimal_number() { parse_number_i64(digits, 10) })
        ) long_number_suffix() { number }

        /// Sequence corresponding to a floating point number's
        /// significand consisting of integer and decimal parts
        /// delimited with a [decimal separators](decimal_separator).
        ///
        /// # Arguments
        ///
        /// * `number` - rule matching valid number for significand's integer and decinal parts
        rule float_number_significand(number: rule<&'input str>)
            -> (Option<&'input str>, Option<&'input str>) = (
                integer_digits:number()? decimal_separator() fractional_digits:number()
                { (integer_digits, Some(fractional_digits)) }
            ) / (
                integer_digits:number() decimal_separator() fractional_digits:(number())?
                { (Some(integer_digits), fractional_digits) }
            );

        rule float_number_hex_significand() -> (Option<&'input str>, Option<&'input str>)
            = float_number_significand(<hex_number()>)

        rule float_number_decimal_significand() -> (Option<&'input str>, Option<&'input str>)
            = float_number_significand(<decimal_number()>)

        /// Number of type `float`
        pub rule float_number() -> Result<f32, ParseError> = number:(
            (number:(
                (
                    hex_number_prefix()
                    significand:float_number_hex_significand()
                    hex_exponent_indicator() exponent:signed_number()
                    { parse_f32_from_parts(significand.0, significand.1, Some(exponent), 16) }
                ) / (
                    significand:float_number_decimal_significand()
                    exponent:(decimal_exponent_indicator() exponent:signed_number() { exponent })?
                    { parse_f32_from_parts(significand.0, significand.1, exponent, 10) }
                )
            ) { number }) / number:(digits:decimal_number(){ parse_number_f32(digits) }) { number }
        ) float_number_suffix() { number }

        /// Number of type `double`
        pub rule double_number() -> Result<f64, ParseError> = (
            (number:(
                (
                    hex_number_prefix()
                    significand:float_number_hex_significand()
                    hex_exponent_indicator() exponent:signed_number()
                    { parse_f64_from_parts(significand.0, significand.1, Some(exponent), 16) }
                ) / (
                    significand:float_number_decimal_significand()
                    exponent:(decimal_exponent_indicator() exponent:signed_number() { exponent })?
                    { parse_f64_from_parts(significand.0, significand.1, exponent, 10) }
                )
            ) double_number_suffix()? { number }) / (
                digits:decimal_number() double_number_suffix()
                { parse_number_f64(digits) }
            )
        )

        pub rule character_value() -> ast::Char = (
            "'" value:(
                value:("\\" value:(
                    (
                        "u" digits:$(hex_digit()*<4,4>)
                        { ast::Char::from_str_radix(digits, 16).unwrap() }
                    )
                    / (
                        digits:($(['0'..='3'] octal_digit() octal_digit()) / $(octal_digit()*<1,2>))
                        { ast::Char::from_str_radix(digits, 8).unwrap() }
                    ) / (value:escape_sequence() { value })
                ) { value }) { value }
                / value:not_escaped_char_symbol() { value as ast::Char }
            ) "'" { value }
        )

        /// Boolean value i.e. `true` or `false`
        pub rule boolean_value() -> ast::Boolean = "true" { true } / "false" { false }

        /// Simply `null` also known as billion-dollar mistake
        pub rule null() = "null";

        // Literals as AST Expressions

        // TODO better error handling instead of `unwrap()`

        /// Literal of type `int`
        pub rule int_literal() -> ast::Expression = value:int_number() {
            ast::Expression::Literal(ast::Literal::Int(value.unwrap()))
        }

        /// Literal of type `long`
        pub rule long_literal() -> ast::Expression = value:long_number() {
            ast::Expression::Literal(ast::Literal::Long(value.unwrap()))
        }

        /// Literal of type `float`
        pub rule float_literal() -> ast::Expression = value:float_number() {
            ast::Expression::Literal(ast::Literal::Float(value.unwrap()))
        }

        /// Literal of type `double`
        pub rule double_literal() -> ast::Expression = value:double_number() {
            ast::Expression::Literal(ast::Literal::Double(value.unwrap()))
        }

        /// Literal of type `boolean`
        pub rule boolean_literal() -> ast::Expression = value:boolean_value() {
            ast::Expression::Literal(ast::Literal::Boolean(value))
        }

        /// Literal of type `char`
        pub rule char_literal() -> ast::Expression = value:character_value() {
            ast::Expression::Literal(ast::Literal::Char(value))
        }

        /// `null` literal
        pub rule null_literal() -> ast::Expression = null() {
            ast::Expression::Literal(ast::Literal::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::java;

    macro_rules! assert_int_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(java::int_number($code).unwrap(), Ok($literal));
        };
        ($literal:expr) => {
            assert_int_number_ok!(stringify!($literal), $literal);
        };
    }

    macro_rules! assert_int_number_err {
        ($code:expr) => {
            assert!(matches!(java::int_number($code).unwrap(), Err(_)));
        };
    }

    #[test]
    fn int_hex_number() {
        assert_int_number_ok!(0x0);
        assert_int_number_ok!(0x00);
        assert_int_number_ok!(0x0000);
        assert_int_number_ok!(0xCAFE);
        assert_int_number_ok!(0xCAFE);
        assert_int_number_ok!("0xFaceB00c", 0xFaceB00cu32 as i32);
        assert_int_number_ok!("0xFace_B00c", 0xFace_B00Cu32 as i32);
        assert_int_number_ok!("0xCAFEBABE", 0xCAFEBABEu32 as i32);
        assert_int_number_ok!("0xFFFFFFFF", 0xFFFFFFFFu32 as i32);

        assert_int_number_err!("0x100000000");
        assert_int_number_err!("0xCAFEBABE0");
        assert_int_number_err!("0xBABEBABEBABEBABE");
    }

    #[test]
    fn int_binary_number() {
        assert_int_number_ok!(0b0);
        assert_int_number_ok!(0b00);
        assert_int_number_ok!(0b0000);
        assert_int_number_ok!(0b1010010101010);
        assert_int_number_ok!(0b1111111111);
        assert_int_number_ok!(
            "0b11111111111111111111111111111111",
            0b11111111111111111111111111111111u32 as i32
        );

        assert_int_number_err!("0b100000000000000000000000000000001");
        assert_int_number_err!("0b100001000010010010000111000010001");
    }

    #[test]
    fn int_octal_number() {
        assert_int_number_ok!("00", 0o0);
        assert_int_number_ok!("000", 0o00);
        assert_int_number_ok!("00000", 0o0000);
        assert_int_number_ok!("01201241", 0o1201241);
        assert_int_number_ok!("01020143176", 0o1020143176);
        assert_int_number_ok!("037777777777", 0o37777777777u32 as i32);

        assert_int_number_err!("047777777777");
    }

    #[test]
    fn int_decimal_number() {
        assert_int_number_ok!(0);
        assert_int_number_ok!(1);
        assert_int_number_ok!(9752);
        assert_int_number_ok!(97521254);
        // Note: the number will be a negative integer equal to
        assert_int_number_ok!("2147483648", 2147483648u32 as i32);
        assert_int_number_ok!("2147483648", i32::MIN);

        assert_int_number_ok!(i32::MAX.to_string().as_str(), i32::MAX);
        assert_int_number_err!(format!("{}0", i32::MAX).as_str());
    }

    macro_rules! assert_long_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(java::long_number($code).unwrap(), Ok($literal));
        };
        ($literal:expr) => {
            assert_long_number_ok!(stringify!($literal), $literal);
            assert_long_number_ok!(concat!(stringify!($literal), "l"), $literal);
            assert_long_number_ok!(concat!(stringify!($literal), "L"), $literal);
        };
    }

    macro_rules! assert_long_number_err {
        ($code:expr) => {
            assert!(matches!(java::long_number($code).unwrap(), Err(_)));
        };
    }

    #[test]
    fn long_hex_number() {
        assert_long_number_ok!("0x0L", 0x0);
        assert_long_number_ok!("0x00L", 0x00);
        assert_long_number_ok!("0x0000L", 0x0000);
        assert_long_number_ok!("0xFaceB00cL", 0xFaceB00c);
        assert_long_number_ok!("0xFace_B00cL", 0xFace_B00C);
        assert_long_number_ok!("0xCAFEBABEDEADL", 0xCAFEBABEDEAD);
        assert_long_number_ok!("0xCAFE_BABE_DEADL", 0xCAFE_BABE_DEAD);
        assert_long_number_ok!("0xCAFEBABE_DEADBEEFL", 0xCAFEBABE_DEADBEEFu64 as i64);

        assert_long_number_err!("0xCAFEBABE_DEADBEEFFL");
    }

    #[test]
    fn long_binary_number() {
        assert_long_number_ok!("0b0L", 0b0);
        assert_long_number_ok!("0b00L", 0b00);
        assert_long_number_ok!("0b0000L", 0b0000);
        assert_long_number_ok!("0b1010010101010L", 0b1010010101010);
        assert_long_number_ok!("0b1111111111L", 0b1111111111);
        assert_long_number_ok!(
            "0b1111111111111111111111111111111111111111111111111111111111111111L",
            0b1111111111111111111111111111111111111111111111111111111111111111u64 as i64
        );

        assert_long_number_err!(
            "0b10000000000000000000000000000000000000000000000000000000000000001L"
        );
    }

    #[test]
    fn long_octal_number() {
        assert_long_number_ok!("00L", 0o0);
        assert_long_number_ok!("000L", 0o00);
        assert_long_number_ok!("00000L", 0o0000);
        assert_long_number_ok!("01201241L", 0o1201241);
        assert_long_number_ok!("01020143176L", 0o1020143176);

        assert_long_number_ok!("0777777777777777777777L", 0o777777777777777777777u64 as i64);

        assert_long_number_err!("07777777777777777777770L");
        assert_long_number_err!("07777777777777777777777L");
    }

    #[test]
    fn long_decimal_number() {
        assert_long_number_ok!("0L", 0);
        assert_long_number_ok!("1L", 1);
        assert_long_number_ok!("9752L", 9752);
        assert_long_number_ok!("97521254L", 97521254);
        assert_long_number_ok!("11057130957130L", 11057130957130);
        assert_long_number_ok!("9223372036854775808L", 9223372036854775808u64 as i64);
        assert_long_number_ok!("9223372036854775808L", i64::MIN);

        assert_long_number_ok!("9223372036854775807L", i64::MAX);
        assert_long_number_err!(format!("{}0L", i64::MAX).as_str());
    }

    macro_rules! assert_float_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(java::float_number($code).unwrap(), Ok($literal));
        };
        ($literal:expr) => {
            assert_float_number_ok!(concat!(stringify!($literal), "f"), $literal);
            assert_float_number_ok!(concat!(stringify!($literal), "F"), $literal);
        };
    }

    #[test]
    fn float_decimal_e_number() {
        assert_float_number_ok!(1.2E3);
        assert_float_number_ok!(1.2213E-7);
        assert_float_number_ok!(0.1248762174E-99);
        assert_float_number_ok!(12.34e+7);
        assert_float_number_ok!(12.34e+7);
    }

    #[test]
    fn float_hex_e_number() {
        assert_float_number_ok!("0xA.Bp1f", 10.11e1);
        assert_float_number_ok!("0xA.Bp1F", 10.11e1);

        assert_float_number_ok!("0x2D.Fp+5f", 45.15e+5);
        assert_float_number_ok!("0x2D.Fp+5F", 45.15e+5);
    }

    #[test]
    fn float_point_number() {
        assert_float_number_ok!(0.123);
        // TODO: fix big numerics
        //assert_float_number_ok!(7498127648197589127581591285789175921.12879491749812748291742948);

        assert_float_number_ok!(890.);
        assert_float_number_ok!(281937128947128921.);

        assert_float_number_ok!(".4567f", 0.4567);
        assert_float_number_ok!(".4567F", 0.4567);
        assert_float_number_ok!(".912640821765892165f", 0.912640821765892165);
        assert_float_number_ok!(".912640821765892165F", 0.912640821765892165);
    }

    #[test]
    fn float_prefix_number() {
        assert_float_number_ok!("0f", 0f32);
        assert_float_number_ok!("0F", 0f32);

        assert_float_number_ok!("000f", 0f32);
        assert_float_number_ok!("000F", 0f32);

        assert_float_number_ok!("123f", 123f32);
        assert_float_number_ok!("123F", 123f32);

        assert_float_number_ok!("123f", 123f32);
        assert_float_number_ok!("123F", 123f32);

        assert_float_number_ok!(
            "9999999999999999999999999999f",
            9999999999999999999999999999f32
        );
        assert_float_number_ok!(
            "9999999999999999999999999999F",
            9999999999999999999999999999f32
        );
    }

    macro_rules! assert_double_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(java::double_number($code).unwrap(), Ok($literal));
        };
        ($literal:expr) => {
            assert_double_number_ok!(stringify!($literal), $literal);
            assert_double_number_ok!(concat!(stringify!($literal), "d"), $literal);
            assert_double_number_ok!(concat!(stringify!($literal), "D"), $literal);
        };
    }

    #[test]
    fn double_decimal_e_number() {
        assert_double_number_ok!(1.2E3);
        assert_double_number_ok!(1.2213E-7);
        assert_double_number_ok!(0.1248762174E-99);
        assert_double_number_ok!(12.34e+56);
    }

    #[test]
    fn double_hex_e_number() {
        assert_double_number_ok!("0xA.Bp1", 10.11e1);
        assert_double_number_ok!("0xA.Bp1d", 10.11e1);
        assert_double_number_ok!("0xA.Bp1D", 10.11e1);

        assert_double_number_ok!("0x2D.Fp+5", 45.15e+5);
        assert_double_number_ok!("0x2D.Fp+5d", 45.15e+5);
        assert_double_number_ok!("0x2D.Fp+5D", 45.15e+5);
    }

    #[test]
    fn double_point_number() {
        assert_double_number_ok!(0.123);
        // TODO: fix big numerics
        //assert_double_number_ok!(7498127648197589127581591285789175921.12879491749812748291742948);

        assert_double_number_ok!(890.);
        // TODO: fix big numerics
        //assert_double_number_ok!(821748912784907120470215012759201587129057219057291075.);

        assert_double_number_ok!(".4567", 0.4567);
        assert_double_number_ok!(".4567d", 0.4567);
        assert_double_number_ok!(".4567D", 0.4567);

        assert_double_number_ok!(".912640821765892160", 0.912640821765892160);
        assert_double_number_ok!(".912640821765892160d", 0.912640821765892160);
        assert_double_number_ok!(".912640821765892160D", 0.912640821765892160);
    }

    #[test]
    fn double_prefix_number() {
        assert_double_number_ok!("0d", 0f64);
        assert_double_number_ok!("0D", 0f64);

        assert_double_number_ok!("000d", 0f64);
        assert_double_number_ok!("000D", 0f64);

        assert_double_number_ok!("123d", 123f64);
        assert_double_number_ok!("123D", 123f64);

        assert_double_number_ok!("123d", 123f64);
        assert_double_number_ok!("123D", 123f64);

        assert_double_number_ok!(
            "9999999999999999999999999999d",
            9999999999999999999999999999f64
        );
        assert_double_number_ok!(
            "9999999999999999999999999999D",
            9999999999999999999999999999f64
        );
    }

    macro_rules! assert_character_value_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(java::character_value($code).unwrap(), $literal as u16);
        };
        ($literal:expr) => {
            assert_character_value_ok!(stringify!($literal), $literal);
        };
    }

    #[test]
    fn char_raw() {
        assert_character_value_ok!('0');
        assert_character_value_ok!('5');
        assert_character_value_ok!('a');
        assert_character_value_ok!('z');
        assert_character_value_ok!('A');
        assert_character_value_ok!('Z');
        assert_character_value_ok!('_');
        assert_character_value_ok!('+');
        assert_character_value_ok!('-');
        assert_character_value_ok!('*');
        assert_character_value_ok!('/');
        assert_character_value_ok!(' ');
    }

    #[test]
    fn char_octal() {
        assert_character_value_ok!('\0');
        assert_character_value_ok!("'\\123'", 0o123u16);
        assert_character_value_ok!("'\\372'", 0o372u16);
        assert_character_value_ok!("'\\22'", 0o22u16);
        assert_character_value_ok!("'\\77'", 0o77u16);
    }

    #[test]
    fn char_unicode() {
        assert_character_value_ok!("'\\u1000'", '\u{1000}');
        assert_character_value_ok!("'\\u1234'", '\u{1234}');
        assert_character_value_ok!("'\\u9999'", '\u{9999}');
        assert_character_value_ok!("'\\u0123'", '\u{123}');
    }

    #[test]
    fn char_escaped() {
        assert_character_value_ok!("'\\b'", '\u{8}');
        assert_character_value_ok!("'\\t'", '\u{9}');
        assert_character_value_ok!("'\\n'", '\u{a}');
        assert_character_value_ok!("'\\f'", '\u{c}');
        assert_character_value_ok!("'\\r'", '\u{d}');
        assert_character_value_ok!("'\\\"'", '\u{22}');
        assert_character_value_ok!("'\\''", '\u{27}');
        assert_character_value_ok!("'\\\\'", '\u{5c}');
    }
}
