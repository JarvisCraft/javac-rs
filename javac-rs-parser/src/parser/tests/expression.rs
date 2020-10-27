use super::*;

mod int_literal_expression {
    use super::*;

    macro_rules! assert_int_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(
                java::int_literal_expression($code),
                Ok(ast::Expression::Literal(ast::Literal::Int($literal)))
            );
        };
        ($literal:expr) => {
            assert_int_number_ok!(stringify!($literal), $literal);
        };
    }

    macro_rules! assert_int_number_err {
        ($code:expr) => {
            assert!(java::int_literal_expression($code).is_err());
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
}

mod long_literal_expression {
    use super::*;

    macro_rules! assert_long_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(
                java::long_literal_expression($code),
                Ok(ast::Expression::Literal(ast::Literal::Long($literal)))
            );
        };
        ($literal:expr) => {
            assert_long_number_ok!(stringify!($literal), $literal);
            assert_long_number_ok!(concat!(stringify!($literal), "l"), $literal);
            assert_long_number_ok!(concat!(stringify!($literal), "L"), $literal);
        };
    }

    macro_rules! assert_long_number_err {
        ($code:expr) => {
            assert!(java::long_literal_expression($code).is_err());
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
}

mod float_literal_expression {
    use super::*;

    macro_rules! assert_float_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(
                java::float_literal_expression($code),
                Ok(ast::Expression::Literal(ast::Literal::Float($literal)))
            );
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
}

mod double_literal_expression {
    use super::*;

    macro_rules! assert_double_number_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(
                java::double_literal_expression($code),
                Ok(ast::Expression::Literal(ast::Literal::Double($literal)))
            );
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
        //assert_double_number_ok!(7498127648197589127581591285789175921
        // .12879491749812748291742948);

        assert_double_number_ok!(890.);
        // TODO: fix big numerics
        //assert_double_number_ok!(8217489127849071204702150127592015871
        // 29057219057291075.);

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
}

mod char_literal_expression {
    use super::*;

    macro_rules! assert_character_value_ok {
        ($code:expr, $literal:expr) => {
            assert_eq!(
                java::char_literal_expression($code).unwrap(),
                ast::Expression::Literal(ast::Literal::Char($literal as ast::Char))
            );
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

mod null_literal_expression {
    use super::*;

    #[test]
    fn null_literal() {
        assert_eq!(
            java::null_literal_expression("null"),
            Ok(ast::Expression::Literal(ast::Literal::Null))
        );
    }

    #[test]
    fn incorrect_null_literal() {
        assert!(java::null_literal_expression("zero").is_err());
        assert!(java::null_literal_expression("nullable").is_err());
        assert!(java::null_literal_expression("0").is_err());
    }
}

mod literal_expression {
    use super::*;

    macro_rules! assert_literal_expression_ok {
        ($code:expr, $expression_name:ident($expression_value:expr)) => {
            assert_eq!(
                java::literal_expression($code),
                Ok(ast::Expression::Literal(ast::Literal::$expression_name(
                    $expression_value
                )))
            );
        };
        ($code:expr, $expression_name:ident) => {
            assert_eq!(
                java::literal_expression($code),
                Ok(ast::Expression::Literal(ast::Literal::$expression_name))
            );
        };
    }

    macro_rules! assert_literal_expression_err {
        ($code:expr) => {
            assert!(java::literal_expression($code).is_err());
        };
    }

    #[test]
    fn literal() {
        assert_literal_expression_ok!("1", Int(1));
        assert_literal_expression_ok!("3.25", Double(3.25));
        assert_literal_expression_ok!("88005553535L", Long(88005553535));
        assert_literal_expression_ok!("1l", Long(1));
        assert_literal_expression_ok!("1L", Long(1));
        assert_literal_expression_ok!("null", Null);
        assert_literal_expression_ok!("true", Boolean(true));
        assert_literal_expression_ok!("false", Boolean(false));
    }

    #[test]
    fn incorrect_literal() {
        assert_literal_expression_err!("1 + 1");
        assert_literal_expression_err!("hi");
        assert_literal_expression_err!("what the hell");
        assert_literal_expression_err!("true boy");
        assert_literal_expression_err!("123man");
    }
}

mod keyword_expression {
    use super::*;

    macro_rules! assert_keyword_expression_ok {
        ($code:expr, $literal:ident) => {
            assert_eq!(
                java::keyword_expression($code),
                Ok(ast::Expression::Keyword(ast::Keyword::$literal))
            );
        };
    }

    macro_rules! assert_keyword_expression_err {
        ($code:expr) => {
            assert!(java::keyword_expression($code).is_err());
        };
    }

    #[test]
    fn keyword() {
        //<editor-fold desc="List of keywords" defaultstate="collapsed">
        assert_keyword_expression_ok!("abstract", Abstract);
        assert_keyword_expression_ok!("assert", Assert);
        assert_keyword_expression_ok!("boolean", Boolean);
        assert_keyword_expression_ok!("break", Break);
        assert_keyword_expression_ok!("byte", Byte);
        assert_keyword_expression_ok!("case", Case);
        assert_keyword_expression_ok!("catch", Catch);
        assert_keyword_expression_ok!("char", Char);
        assert_keyword_expression_ok!("class", Class);
        assert_keyword_expression_ok!("const", Const);
        assert_keyword_expression_ok!("continue", Continue);
        assert_keyword_expression_ok!("default", Default);
        assert_keyword_expression_ok!("do", Do);
        assert_keyword_expression_ok!("double", Double);
        assert_keyword_expression_ok!("else", Else);
        assert_keyword_expression_ok!("enum", Enum);
        assert_keyword_expression_ok!("extends", Extends);
        assert_keyword_expression_ok!("final", Final);
        assert_keyword_expression_ok!("finally", Finally);
        assert_keyword_expression_ok!("float", Float);
        assert_keyword_expression_ok!("for", For);
        assert_keyword_expression_ok!("goto", Goto);
        assert_keyword_expression_ok!("if", If);
        assert_keyword_expression_ok!("implements", Implements);
        assert_keyword_expression_ok!("import", Import);
        assert_keyword_expression_ok!("instanceof", Instanceof);
        assert_keyword_expression_ok!("int", Int);
        assert_keyword_expression_ok!("interface", Interface);
        assert_keyword_expression_ok!("long", Long);
        assert_keyword_expression_ok!("native", Native);
        assert_keyword_expression_ok!("new", New);
        assert_keyword_expression_ok!("package", Package);
        assert_keyword_expression_ok!("private", Private);
        assert_keyword_expression_ok!("protected", Protected);
        assert_keyword_expression_ok!("public", Public);
        assert_keyword_expression_ok!("return", Return);
        assert_keyword_expression_ok!("short", Short);
        assert_keyword_expression_ok!("static", Static);
        assert_keyword_expression_ok!("strictfp", Strictfp);
        assert_keyword_expression_ok!("super", Super);
        assert_keyword_expression_ok!("switch", Switch);
        assert_keyword_expression_ok!("synchronized", Synchronized);
        assert_keyword_expression_ok!("this", This);
        assert_keyword_expression_ok!("throw", Throw);
        assert_keyword_expression_ok!("throws", Throws);
        assert_keyword_expression_ok!("transient", Transient);
        assert_keyword_expression_ok!("try", Try);
        assert_keyword_expression_ok!("void", Void);
        assert_keyword_expression_ok!("volatile", Volatile);
        assert_keyword_expression_ok!("while", While);
        //</editor-fold>
    }

    #[test]
    fn incorrect_keyword() {
        assert_keyword_expression_err!("integer");
        assert_keyword_expression_err!("while ago");
        assert_keyword_expression_err!("nonvolatile");
        assert_keyword_expression_err!("throwable");
    }
}

mod identifier_expression {
    use super::*;

    macro_rules! assert_identifier_expression_ok {
        ($code:expr) => {
            assert_eq!(
                java::identifier_expression($code),
                Ok(ast::Expression::Identifier($code.to_string()))
            );
        };
    }

    macro_rules! assert_identifier_expression_err {
        ($code:expr) => {
            assert!(java::identifier_expression($code).is_err());
        };
    }

    #[test]
    fn identifier() {
        assert_identifier_expression_ok!("hello");
        assert_identifier_expression_ok!("wow");
        assert_identifier_expression_ok!("oma1ga1");
        assert_identifier_expression_ok!("$tonks");
        assert_identifier_expression_ok!("$$$");
        assert_identifier_expression_ok!("$12$34$");
        assert_identifier_expression_ok!("$12$34$56");
    }

    #[test]
    fn incorrect_identifier() {
        assert_identifier_expression_err!("abstract");
        assert_identifier_expression_err!("static");
        assert_identifier_expression_err!("final");
        assert_identifier_expression_err!("int");
        assert_identifier_expression_err!("finally");
        assert_identifier_expression_err!("8800");
        assert_identifier_expression_err!("1man");
        assert_identifier_expression_err!("hi bro");
        assert_identifier_expression_err!("qq\nfriend");
    }
}

mod comment_expression {
    use super::*;

    macro_rules! assert_comment_expression_ok {
        ($code:expr, $body:expr) => {
            assert_eq!(
                java::comment_expression($code),
                Ok(ast::Expression::Comment($body.to_string()))
            );
        };
    }

    macro_rules! assert_comment_expression_err {
        ($code:expr) => {
            assert!(java::comment_expression($code).is_err());
        };
    }

    #[test]
    fn inline_comment() {
        assert_comment_expression_ok!("//Test", "Test");
        assert_comment_expression_ok!("// Hello world", " Hello world");
        assert_comment_expression_ok!("//", "");
        assert_comment_expression_ok!("//Hello\n", "Hello");
    }

    #[test]
    fn incorrect_inline_comment() {
        assert_comment_expression_err!("/ /Roses");
        assert_comment_expression_err!("\\\\Violins");
        assert_comment_expression_err!("/\\Unexpected");
        assert_comment_expression_err!("\\/32");
    }

    #[test]
    fn multiline_comment() {
        assert_comment_expression_ok!("/*Smol*/", "Smol");
        assert_comment_expression_ok!("/* Potat */", " Potat ");
        assert_comment_expression_ok!(
            "/*\n\
                Hello\r\n\
                World\n\
                */",
            "\nHello\r\nWorld\n"
        );
    }

    #[test]
    fn incorrect_multiline_comment() {
        assert_comment_expression_err!("*/ ohno */");
        assert_comment_expression_err!("/* omagad /*");
        assert_comment_expression_err!("/* just non-terminated");
        assert_comment_expression_err!("WTJ */");
        assert_comment_expression_err!("/*\n");
    }
}
