pub use peg;

pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

// TODO remove unneeded pub's
peg::parser! {
    /// Java language grammar as specified by [JLS](https://docs.oracle.com/javase/specs/)
    pub grammar java() for str {

        use std::num::ParseIntError;
        use std::num::ParseFloatError;

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

        /// Optional separator of digits in numbers
        pub rule digit_separator() = "_"

        /// Sequence of specified digits with optional
        /// non-trailing [digit separators](digit_separator.
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

        /// Number of type `int`
        pub rule int_number() -> Result<i32, ParseIntError> = (
            (hex_number_prefix() digits:hex_number() {
                 u32::from_str_radix(digits.replace('_', "").as_str(), 16)
                .map(|value| {value as i32})
            }) / (binary_number_prefix() digits:binary_number() {
                u32::from_str_radix(digits.replace('_', "").as_str(), 2)
                .map(|value| {value as i32})
            }) / (octal_number_prefix() digits:octal_number() {
                u32::from_str_radix(digits.replace('_', "").as_str(), 8)
                .map(|value| {value as i32})
            }) / (digits:decimal_number() {
                u32::from_str_radix(digits.replace('_', "").as_str(), 10)
                .map(|value| {value as i32})
            })
        )

        /// Number of type `long`
        pub rule long_number() -> Result<i64, ParseIntError> = number:(
            (hex_number_prefix() digits:hex_number() {
                u64::from_str_radix(digits.replace('_', "").as_str(), 16)
                .map(|value| {value as i64})
            }) / (binary_number_prefix() digits:binary_number() {
                u64::from_str_radix(digits.replace('_', "").as_str(), 2)
                .map(|value| {value as i64})
            }) / (octal_number_prefix() digits:octal_number() {
                u64::from_str_radix(digits.replace('_', "").as_str(), 8)
                .map(|value| {value as i64})
            }) / (digits:decimal_number() {
                u64::from_str_radix(digits.replace('_', "").as_str(), 10)
                .map(|value| {value as i64})
            })
        ) long_number_suffix() { number }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::java;

    macro_rules! assert_int_number_ok {
        ($original:expr, $parsed:expr) => {
            assert_eq!(java::int_number($original).unwrap(), Ok($parsed))
        }
    }

    macro_rules! assert_int_number_err {
        ($original:expr) => {
            assert!(matches!(java::int_number($original).unwrap(), Err(_)))
        }
    }

    #[test]
    fn int_hex_number() {
        assert_int_number_ok!("0x0", 0x0);
        assert_int_number_ok!("0x00", 0x00);
        assert_int_number_ok!("0x0000", 0x0000);
        assert_int_number_ok!("0xCA_FE", 0xCAFE);
        assert_int_number_ok!("0xCAFE", 0xCAFE);
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
        assert_int_number_ok!("0b0", 0b0);
        assert_int_number_ok!("0b00", 0b00);
        assert_int_number_ok!("0b0000", 0b0000);
        assert_int_number_ok!("0b1010010101010", 0b1010010101010);
        assert_int_number_ok!("0b1111111111", 0b1111111111);
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
        assert_int_number_ok!("0", 0);
        assert_int_number_ok!("1", 1);
        assert_int_number_ok!("9752", 9752);
        assert_int_number_ok!("97521254", 97521254);

        assert_int_number_ok!(i32::MAX.to_string().as_str(), i32::MAX);
        assert_int_number_err!(format!("{}0", i32::MAX).as_str());
    }

    macro_rules! assert_long_number_ok {
        ($original:expr, $parsed:expr) => {
            assert_eq!(java::long_number($original).unwrap(), Ok($parsed))
        }
    }

    macro_rules! assert_long_number_err {
        ($original:expr) => {
            assert!(matches!(java::long_number($original).unwrap(), Err(_)))
        }
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

        assert_long_number_ok!("9223372036854775807L", i64::MAX);
        assert_long_number_err!(format!("{}0L", i64::MAX).as_str());
    }
}
