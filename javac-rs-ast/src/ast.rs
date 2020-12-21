/// Java `boolean` type
pub type Boolean = bool;
/// Java `byte` type
pub type Byte = i8;
/// Java `short` type
pub type Short = i16;
/// Java `char` type
pub type Char = u16;
/// Java `int` type
pub type Int = i32;
/// Java `long` type
pub type Long = i64;
/// Java `float` type
pub type Float = f32;
/// Java `double` type
pub type Double = f64;

/// Type used for storing raw identifiers
pub type IdentifierName = String;

/// Type used for storing raw comment body
pub type CommentBody = String;

/// Type used for storing raw string literal body
pub type StringLiteralValue = String;

/// Java [literal](https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html#jls-3.10)
#[derive(Debug, PartialEq)]
pub enum Literal {
    /// Literal of `int` type
    Int(Int),
    /// Literal of `long` type
    Long(Long),
    /// Literal of `float` type
    Float(Float),
    /// Literal of `double` type
    Double(Double),
    /// Literal of `boolean` type
    Boolean(Boolean),
    /// Literal of `char` type
    Char(Char),
    /// Literal of `java.lang.String` type
    String(StringLiteralValue),
    /// `null` literal
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    //<editor-fold desc="List of keywords" defaultstate="collapsed">
    Abstract,
    Assert,
    Boolean,
    Break,
    Byte,
    Case,
    Catch,
    Char,
    Class,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extends,
    Final,
    Finally,
    Float,
    For,
    Goto,
    If,
    Implements,
    Import,
    Instanceof,
    Int,
    Interface,
    Long,
    Native,
    New,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Short,
    Static,
    Strictfp,
    Super,
    Switch,
    Synchronized,
    This,
    Throw,
    Throws,
    Transient,
    Try,
    Void,
    Volatile,
    While,
    //</editor-fold>
}

impl Keyword {
    /// Checks if this keyword represents a (primitive) type.
    ///
    /// # Examples
    ///
    /// ```
    /// use javac_rs_ast::ast::Keyword;
    ///
    /// assert!(Keyword::Byte.is_type());
    /// assert!(!Keyword::Private.is_type());
    /// assert!(!Keyword::Synchronized.is_type());
    /// ```
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            Self::Boolean
                | Self::Byte
                | Self::Char
                | Self::Double
                | Self::Float
                | Self::Int
                | Self::Long
                | Self::Short
        )
    }

    /// Checks if this keyword represents an access modifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use javac_rs_ast::ast::Keyword;
    ///
    /// assert!(Keyword::Public.is_access_modifier());
    /// assert!(!Keyword::Package.is_access_modifier());
    /// assert!(!Keyword::Continue.is_access_modifier());
    /// ```
    pub fn is_access_modifier(&self) -> bool {
        matches!(self, Self::Private | Self::Protected | Self::Public)
    }

    /// Checks if this keyword represents an unused keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use javac_rs_ast::ast::Keyword;
    ///
    /// assert!(Keyword::Const.is_unused());
    /// assert!(Keyword::Goto.is_unused());
    /// assert!(!Keyword::Abstract.is_unused());
    /// assert!(!Keyword::Case.is_unused());
    /// ```
    pub fn is_unused(&self) -> bool {
        matches!(self, Self::Const | Self::Goto)
    }
}

/// A Java expression in source code AST
#[derive(Debug, PartialEq)]
pub enum Expression {
    /// Keyword expression
    Keyword(Keyword),
    /// Literal expression
    Literal(Literal),
    /// Identifier expression
    Identifier(IdentifierName),
    /// Comment expression
    Comment(CommentBody),
}
