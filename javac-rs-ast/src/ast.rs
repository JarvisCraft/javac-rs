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

/// Java [literal](https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html#jls-3.10)
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
    /// Literal of `double` type
    String(String),
    /// `null` literal
    Null,
}

/// A Java expression in source code AST
pub enum Expression {
    /// Literal expression
    Literal(Literal),
}
