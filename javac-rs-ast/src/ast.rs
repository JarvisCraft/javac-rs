/// Java `boolean` type
type Boolean = bool;
/// Java `byte` type
type Byte = i8;
/// Java `short` type
type Short = i16;
/// Java `char` type
type Char = u16;
/// Java `int` type
type Int = i32;
/// Java `long` type
type Long = i32;
/// Java `float` type
type Float = f32;
/// Java `double` type
type Double = f64;

/// Java [literal](https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html#jls-3.10)
pub enum Literal {
    /// `null` literal
    Null,
    /// Literal of `boolean` type
    Boolean(Boolean),
    /// Literal of `int` type
    Int(Int),
    /// Literal of `long` type
    Long(Long),
    /// Literal of `float` type
    Float(Float),
    /// Literal of `double` type
    Double(Double),
}

/// A Java expression in source code AST
pub enum Expression {
    /// Literal expression
    Literal(Literal),
}
