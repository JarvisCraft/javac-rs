pub use javac_rs_ast::ast;
pub use peg;

mod literals;

peg::parser! {
    /// Java language grammar as specified by [JLS](https://docs.oracle.com/javase/specs/)
    pub grammar java() for str {

        // Basic lexical objects not bound to AST-nodes

        /// Digit of a binary number
        rule binary_digit() -> char
                = slice:$(['0' | '1']) { slice.chars().next().unwrap() }

        /// Digit of an octal number
        rule octal_digit() -> char
                = slice:$(['0'..='7']) { slice.chars().next().unwrap() }

        /// Digit of a decimal number
        rule decimal_digit() -> char
                = slice:$(['0'..='9']) { slice.chars().next().unwrap() }

        /// Digit of a hex number
        rule hex_digit() -> char
                = slice:$(['0'..='9' | 'A'..='F' | 'a'..='f']) { slice.chars().next().unwrap() }

        /// Prefix of a binary number
        rule binary_number_prefix() = "0" ['B' | 'b']

        /// Prefix of an octal number
        rule octal_number_prefix() = "0"

        /// Prefix of a hex number
        rule hex_number_prefix() = "0" ['X' | 'x']

        /// Suffix of `long` number
        rule long_number_suffix() = ['L' | 'l']

        /// Suffix of `float` number
        rule float_number_suffix() = ['F' | 'f']

        /// Suffix of `double` number
        rule double_number_suffix() = ['D' | 'd']

        /// Indicator of a start of the exponent of a decimal number
        rule decimal_exponent_indicator() = ['E' | 'e']

        /// Indicator of a start of the exponent of a hex number
        rule hex_exponent_indicator() = ['P' | 'p']

        /// Rule returning `-1i32` for `"-"` and `+1i32` for anything other
        rule sign_num_i32() -> i32 = "-" { -1 } / { 1 }

        /// Rule returning `-1i64` for `"-"` and `+1i64` for anything other
        rule sign_num_i64() -> i64 = "-" { -1 } / { 1 }

        /// Optional separator of digits in numbers
        rule digit_separator() = "_"

        /// Separator of integer and fractional parts of numbers
        rule decimal_separator() = "."

        /// A character which does not require escaping in char literals
        rule not_escaped_char_symbol() -> char
            = character:$(!['\\' | '\''][_]) { character.chars().next().unwrap() };

        rule escape_sequence() -> ast::Char
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
        rule int_number() -> i32
            = (hex_number_prefix() digits:hex_number() {?
                literals::parse_number_i32(digits, 16)
            }) / (binary_number_prefix() digits:binary_number() {?
                literals::parse_number_i32(digits, 2)
            }) / (octal_number_prefix() digits:octal_number() {?
                literals::parse_number_i32(digits, 8)
            }) / (digits:decimal_number() {?
                literals::parse_number_i32(digits, 10)
            })

        /// Number of type `long`
        rule long_number() -> i64 = number:(
            (hex_number_prefix() digits:hex_number() {?
                literals::parse_number_i64(digits, 16)
            }) / (binary_number_prefix() digits:binary_number() {?
                literals::parse_number_i64(digits, 2)
            }) / (octal_number_prefix() digits:octal_number() {?
                literals::parse_number_i64(digits, 8)
            }) / (digits:decimal_number() {?
                literals::parse_number_i64(digits, 10)
            })
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
        rule float_number() -> f32 = number:(
            (
                hex_number_prefix()
                significand:float_number_hex_significand()
                hex_exponent_indicator() exponent:signed_number() {?
                    literals::parse_from_parts::<f32>(
                        significand.0, significand.1, Some(exponent), 16
                    )
                }
            ) / (
                significand:float_number_decimal_significand()
                exponent:(decimal_exponent_indicator() exponent:signed_number() { exponent })? {?
                    literals::parse_from_parts::<f32>(
                        significand.0, significand.1, exponent, 10
                    )
                }
            ) / (digits:decimal_number() {? literals::parse_floating_point_number(digits) })
        ) float_number_suffix() { number }

        /// Number of type `double`
        rule double_number() -> f64 = (number:((
            hex_number_prefix()
            significand:float_number_hex_significand()
            hex_exponent_indicator() exponent:signed_number() {?
                literals::parse_from_parts::<f64>(
                    significand.0, significand.1, Some(exponent), 16
                )
            }
        ) / (
            significand:float_number_decimal_significand()
            exponent:(decimal_exponent_indicator() exponent:signed_number() { exponent })? {?
                literals::parse_from_parts::<f64>(significand.0, significand.1, exponent, 10)
            }
        )) double_number_suffix()? { number }) / (
            digits:decimal_number() double_number_suffix()
            {? literals::parse_floating_point_number(digits) }
        )

        rule character_value() -> ast::Char = "'" value:(
            value:("\\" value:(
                ("u" digits:$(hex_digit()*<4,4>) { ast::Char::from_str_radix(digits, 16).unwrap() })
                / (
                    digits:($(['0'..='3'] octal_digit() octal_digit()) / $(octal_digit()*<1,2>))
                    { ast::Char::from_str_radix(digits, 8).unwrap() }
                ) / escape_sequence()
            ) { value }) { value }
            / value:not_escaped_char_symbol() { value as ast::Char }
        ) "'" { value }

        /// Boolean value i.e. `true` or `false`
        rule boolean_value() -> ast::Boolean = "true" { true } / "false" { false }

        /// Simply `null` also known as billion-dollar mistake
        rule null() = "null";

        // Literals as AST Expressions

        /// Literal of type `int`
        pub rule int_literal_expression() -> ast::Expression = value:int_number() {
            ast::Expression::Literal(ast::Literal::Int(value))
        }

        /// Literal of type `long`
        pub rule long_literal_expression() -> ast::Expression = value:long_number() {
            ast::Expression::Literal(ast::Literal::Long(value))
        }

        /// Literal of type `float`
        pub rule float_literal_expression() -> ast::Expression = value:float_number() {
            ast::Expression::Literal(ast::Literal::Float(value))
        }

        /// Literal of type `double`
        pub rule double_literal_expression() -> ast::Expression = value:double_number() {
            ast::Expression::Literal(ast::Literal::Double(value))
        }

        /// Literal of type `boolean`
        pub rule boolean_literal_expression() -> ast::Expression = value:boolean_value() {
            ast::Expression::Literal(ast::Literal::Boolean(value))
        }

        /// Literal of type `char`
        pub rule char_literal_expression() -> ast::Expression = value:character_value() {
            ast::Expression::Literal(ast::Literal::Char(value))
        }

        /// `null` literal
        pub rule null_literal_expression() -> ast::Expression = null() {
            ast::Expression::Literal(ast::Literal::Null)
        }

        pub rule literal_expression() -> ast::Expression
            =  null_literal_expression()
            / char_literal_expression() / boolean_literal_expression()
            / float_literal_expression() / double_literal_expression()
            / long_literal_expression() / int_literal_expression()

        // Identifiers and related

        // TODO non-ascii support
        rule first_identifier_symbol() = ['A'..='Z' | 'a'..='z' | '_' | '$']

        rule identifier_symbol() = decimal_digit() / first_identifier_symbol()

        rule line_terminator() = "\n\r" / ['\n' | '\r']

        rule _() = [' ' | '\t' | '\u{C}'] / line_terminator()

        /// Keyword name as specified by
        /// [JLS 3.9](https://docs.oracle.com/javase/specs/jls/se15/html/jls-3.html#jls-3.9)
        rule keyword() -> ast::Keyword = keyword:(
            //<editor-fold desc="List of keywords" defaultstate="collapsed">
            "abstract" { ast::Keyword::Abstract }
            / "assert" { ast::Keyword::Assert }
            / "boolean" { ast::Keyword::Boolean }
            / "break" { ast::Keyword::Break }
            / "byte" { ast::Keyword::Byte }
            / "case" { ast::Keyword::Case }
            / "catch" { ast::Keyword::Catch }
            / "char" { ast::Keyword::Char }
            / "class" { ast::Keyword::Class }
            / "const" { ast::Keyword::Const }
            / "continue" { ast::Keyword::Continue }
            / "default" { ast::Keyword::Default }
            // [[do]uble] should have higher priority than [do]
            / "double" { ast::Keyword::Double }
            / "do" { ast::Keyword::Do }
            / "else" { ast::Keyword::Else }
            / "enum" { ast::Keyword::Enum }
            / "extends" { ast::Keyword::Extends }
            // [[final]ly] should have higher priority than [final]
            / "finally" { ast::Keyword::Finally }
            / "final" { ast::Keyword::Final }
            / "float" { ast::Keyword::Float }
            / "for" { ast::Keyword::For }
            / "goto" { ast::Keyword::Goto }
            / "if" { ast::Keyword::If }
            / "implements" { ast::Keyword::Implements }
            / "import" { ast::Keyword::Import }
            / "instanceof" { ast::Keyword::Instanceof }
            // [[int]erface] should have higher priority than [int]
            / "interface" { ast::Keyword::Interface }
            / "int" { ast::Keyword::Int }
            / "long" { ast::Keyword::Long }
            / "native" { ast::Keyword::Native }
            / "new" { ast::Keyword::New }
            / "package" { ast::Keyword::Package }
            / "private" { ast::Keyword::Private }
            / "protected" { ast::Keyword::Protected }
            / "public" { ast::Keyword::Public }
            / "return" { ast::Keyword::Return }
            / "short" { ast::Keyword::Short }
            / "static" { ast::Keyword::Static }
            / "strictfp" { ast::Keyword::Strictfp }
            / "super" { ast::Keyword::Super }
            / "switch" { ast::Keyword::Switch }
            / "synchronized" { ast::Keyword::Synchronized }
            / "this" { ast::Keyword::This }
            // [[throw]s] should have higher priority than [throw]
            / "throws" { ast::Keyword::Throws }
            / "throw" { ast::Keyword::Throw }
            / "transient" { ast::Keyword::Transient }
            / "try" { ast::Keyword::Try }
            / "void" { ast::Keyword::Void }
            / "volatile" { ast::Keyword::Volatile }
            / "while" { ast::Keyword::While }
            //</editor-fold>
        ) !identifier_symbol() { keyword }

        pub rule keyword_expression() -> ast::Expression = value:keyword() {
            ast::Expression::Keyword(value)
        }

        rule identifier_raw() = quiet! {
                !keyword() first_identifier_symbol() (identifier_symbol())*
        } / expected!("Identifier")

        rule identifier_name() -> ast::IdentifierName = identifier:$(identifier_raw()) {
            identifier.into()
        }

        pub rule identifier_expression() -> ast::Expression = value:identifier_name() {
            ast::Expression::Identifier(value)
        }

        rule inline_comment_start() = "//"

        rule inline_comment() -> ast::CommentBody
                = inline_comment_start() body:$((!line_terminator() [_])*)
                (line_terminator() / ![_]) { body.into() }

        rule multiline_comment_start() = "/*"

        rule multiline_comment_end() = "*/"

        rule multiline_comment() -> ast::CommentBody
            = quiet! {
                multiline_comment_start()
                body:$((!multiline_comment_end() [_])*)
                (quiet! { multiline_comment_end() })
                { body.into() }
            } / expected!("End of multiline comment: `*/`")

        pub rule comment_expression() -> ast::Expression
            = body:(inline_comment() / multiline_comment()) { ast::Expression::Comment(body) }
    }
}

#[cfg(test)]
mod tests;
