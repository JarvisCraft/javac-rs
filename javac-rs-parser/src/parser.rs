use peg::ParseLiteral;
use javac_rs_ast::Node;

pub use peg;
use std::fmt::Error;

pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

// TODO remove unneeded pub's
peg::parser! {
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
        pub rule binary_bumber_prefix() = "0b"

        /// Prefix of an octal number
        pub rule octal_bumber_prefix() = "0"

        /// Prefix of a hex number
        pub rule hex_bumber_prefix() = "0x"

        /// Suffix of `long` number
        pub rule long_number_suffix() = ['L' | 'l']

        /// Suffix of `float` number
        pub rule float_number_suffix() = ['F' | 'f']

        /// Suffix of `double` number
        pub rule double_number_suffix() = ['D' | 'd']

        /// Optional separator of digits in numbers
        pub rule digit_separator() = "_"

        /// Number of type `int`
        pub rule int_number() -> i64
                = (
                    hex_bumber_prefix()
                    digits:$(hex_digit() (digit_separator()* hex_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 16).unwrap() as i64 }
                ) / (
                    binary_bumber_prefix()
                    digits:$(binary_digit() (digit_separator()* binary_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 2).unwrap() as i64 }
                ) / (
                    octal_bumber_prefix()
                    digits:$(octal_digit() (digit_separator()* octal_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 8).unwrap() as i64 }
                ) / (
                    digits:$(decimal_digit() (digit_separator()* decimal_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 10).unwrap() as i64 }
                )

        /// Number of type `long`
        pub rule long_number() -> i64
                = number:((
                    hex_bumber_prefix()
                    digits:$(hex_digit() (digit_separator()* hex_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 16).unwrap() as i64 }
                ) / (
                    binary_bumber_prefix()
                    digits:$(binary_digit() (digit_separator()* binary_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 2).unwrap() as i64 }
                ) / (
                    octal_bumber_prefix()
                    digits:$(octal_digit() (digit_separator()* octal_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 8).unwrap() as i64 }
                ) / (
                    digits:$(decimal_digit() (digit_separator()* decimal_digit()+)*)
                    { u64::from_str_radix(digits.replace('_', "").as_str(), 10).unwrap() as i64 }
                )) long_number_suffix() { number }
    }
}

#[cfg(test)]
mod tests {
    use javac_rs_ast::Node;
    use crate::parser::java;

    #[test]
    fn long_hex_number() {
        assert_eq!(java::long_number("0x0L"), Ok(0x0));
        assert_eq!(java::long_number("0x00L"), Ok(0x00));
        assert_eq!(java::long_number("0x0000L"), Ok(0x0000));
        assert_eq!(java::long_number("0xFaceB00cL"), Ok(0xFaceB00c));
        assert_eq!(java::long_number("0xFace_B00cL"), Ok(0xFace_B00C));
        assert_eq!(java::long_number("0xCAFEBABEDEADL"), Ok(0xCAFEBABEDEAD));
        assert_eq!(java::long_number("0xCAFE_BABE_DEADL"), Ok(0xCAFE_BABE_DEAD));
        assert_eq!(java::long_number("0xCAFEBABE_DEADBEEFL"), Ok(0xCAFEBABE_DEADBEEFu64 as i64));
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

    #[test]
    fn it_works() {
        assert_eq!(i64::from_str_radix("CAFEBABE", 16).unwrap(), 0xCAFEBABEi64);
    }
}
