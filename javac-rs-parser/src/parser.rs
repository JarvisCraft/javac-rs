pub use peg;

pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

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
        pub rule binary_number_prefix() = "0b"

        /// Prefix of an octal number
        pub rule octal_number_prefix() = "0"

        /// Prefix of a hex number
        pub rule hex_number_prefix() = "0x"

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
        pub rule int_number() -> i32
                = (hex_number_prefix() digits:hex_number() {
                    u32::from_str_radix(digits.replace('_', "").as_str(), 16)
                    .map(|value| {value as i32}).unwrap()
                }) / (binary_number_prefix() digits:binary_number() {
                    u32::from_str_radix(digits.replace('_', "").as_str(), 2)
                    .map(|value| {value as i32}).unwrap()
                }) / (octal_number_prefix() digits:octal_number() {
                    u32::from_str_radix(digits.replace('_', "").as_str(), 8)
                    .map(|value| {value as i32}).unwrap()
                }) / (digits:decimal_number() {
                    u32::from_str_radix(digits.replace('_', "").as_str(), 10)
                    .map(|value| {value as i32}).unwrap()
                })

        /// Number of type `long`
        pub rule long_number() -> i64
                = number:(
                    (hex_number_prefix() digits:hex_number() {
                        u64::from_str_radix(digits.replace('_', "").as_str(), 16)
                        .map(|value| {value as i64}).unwrap()
                    }) / (binary_number_prefix() digits:binary_number() {
                        u64::from_str_radix(digits.replace('_', "").as_str(), 2)
                        .map(|value| {value as i64}).unwrap()
                    }) / (octal_number_prefix() digits:octal_number() {
                        u64::from_str_radix(digits.replace('_', "").as_str(), 8)
                        .map(|value| {value as i64}).unwrap()
                    }) / (digits:decimal_number() {
                        u64::from_str_radix(digits.replace('_', "").as_str(), 10)
                        .map(|value| {value as i64}).unwrap()
                    })
                ) long_number_suffix() { number }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::java;

    #[test]
    fn int_hex_number() {
        assert_eq!(java::int_number("0x0"), Ok(0x0));
        assert_eq!(java::int_number("0x00"), Ok(0x00));
        assert_eq!(java::int_number("0x0000"), Ok(0x0000));
        assert_eq!(java::int_number("0xCA_FE"), Ok(0xCAFE));
        assert_eq!(java::int_number("0xCAFE"), Ok(0xCAFE));
        assert_eq!(java::int_number("0xFaceB00c"), Ok(0xFaceB00cu32 as i32));
        assert_eq!(java::int_number("0xFace_B00c"), Ok(0xFace_B00Cu32 as i32));
        assert_eq!(
            java::int_number("0xCAFEBABE_DEADBEEF"),
            Ok(0xCAFEBABEu32 as i32)
        );
    }

    #[test]
    fn int_binary_number() {
        assert_eq!(java::int_number("0b0"), Ok(0b0));
        assert_eq!(java::int_number("0b00"), Ok(0b00));
        assert_eq!(java::int_number("0b0000"), Ok(0b0000));
        assert_eq!(java::int_number("0b1010010101010"), Ok(0b1010010101010));
        assert_eq!(java::int_number("0b1111111111"), Ok(0b1111111111));
    }

    #[test]
    fn int_octal_number() {
        assert_eq!(java::int_number("00"), Ok(0o0));
        assert_eq!(java::int_number("000"), Ok(0o00));
        assert_eq!(java::int_number("00000"), Ok(0o0000));
        assert_eq!(java::int_number("01201241"), Ok(0o1201241));
        assert_eq!(java::int_number("01020143176"), Ok(0o1020143176));
    }

    #[test]
    fn int_decimal_number() {
        assert_eq!(java::int_number("0"), Ok(0));
        assert_eq!(java::int_number("1"), Ok(1));
        assert_eq!(java::int_number("9752"), Ok(9752));
        assert_eq!(java::int_number("97521254"), Ok(97521254));
        assert_eq!(java::int_number("1013957130"), Ok(1013957130));
    }

    #[test]
    fn long_hex_number() {
        assert_eq!(java::long_number("0x0L"), Ok(0x0));
        assert_eq!(java::long_number("0x00L"), Ok(0x00));
        assert_eq!(java::long_number("0x0000L"), Ok(0x0000));
        assert_eq!(java::long_number("0xFaceB00cL"), Ok(0xFaceB00c));
        assert_eq!(java::long_number("0xFace_B00cL"), Ok(0xFace_B00C));
        assert_eq!(java::long_number("0xCAFEBABEDEADL"), Ok(0xCAFEBABEDEAD));
        assert_eq!(java::long_number("0xCAFE_BABE_DEADL"), Ok(0xCAFE_BABE_DEAD));
        assert_eq!(
            java::long_number("0xCAFEBABE_DEADBEEFL"),
            Ok(0xCAFEBABE_DEADBEEFu64 as i64)
        );
    }

    #[test]
    fn long_binary_number() {
        assert_eq!(java::long_number("0b0L"), Ok(0b0));
        assert_eq!(java::long_number("0b00L"), Ok(0b00));
        assert_eq!(java::long_number("0b0000L"), Ok(0b0000));
        assert_eq!(java::long_number("0b1010010101010L"), Ok(0b1010010101010));
        assert_eq!(java::long_number("0b1111111111L"), Ok(0b1111111111));
    }

    #[test]
    fn long_octal_number() {
        assert_eq!(java::long_number("00L"), Ok(0o0));
        assert_eq!(java::long_number("000L"), Ok(0o00));
        assert_eq!(java::long_number("00000L"), Ok(0o0000));
        assert_eq!(java::long_number("01201241L"), Ok(0o1201241));
        assert_eq!(java::long_number("01020143176L"), Ok(0o1020143176));
    }

    #[test]
    fn long_decimal_number() {
        assert_eq!(java::long_number("0L"), Ok(0));
        assert_eq!(java::long_number("1L"), Ok(1));
        assert_eq!(java::long_number("9752L"), Ok(9752));
        assert_eq!(java::long_number("97521254L"), Ok(97521254));
        assert_eq!(java::long_number("11057130957130L"), Ok(11057130957130));
    }
}
