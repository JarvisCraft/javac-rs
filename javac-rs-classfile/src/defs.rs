//! Generic classfile-specific definitions

/// Header of Java class file
pub const CLASSFILE_HEADER: [u8; 4] = [0xCAu8, 0xFEu8, 0xBAu8, 0xBEu8];

/// Name of a constructor
/// [method](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.9)
pub const CONSTRUCTOR_METHOD_NAME: &str = "<init>";

/// Name of a static initializer
/// [method](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.9)
pub const STATIC_INITIALIZER_METHOD_NAME: &str = "<clinit>";

/// `byte` [type](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.3.1)
pub type JvmByte = i8;

/// `short` [type](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.3.1)
pub type JvmShort = i16;

/// `int` [type](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.3.1)
pub type JvmInt = i32;

/// `long` [type](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.3.1)
pub type JvmLong = i64;

/// `char` [type](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.3.1)
pub type JvmChar = u16;

/// `float` [type](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.3.1)
pub type JvmFloat = f32;

/// `double` [type](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-2.html#jvms-2.3.1)
pub type JvmDouble = f64;
