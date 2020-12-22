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

pub mod major_versions {
    pub const JAVA_1_1: u16 = 45;
    pub const JAVA_1_2: u16 = 46;
    pub const JAVA_1_3: u16 = 47;
    pub const JAVA_1_4: u16 = 48;
    pub const JAVA_5_0: u16 = 49;
    pub const JAVA_6_0: u16 = 50;
    pub const JAVA_7: u16 = 51;
    pub const JAVA_8: u16 = 52;
    pub const JAVA_9: u16 = 53;
    pub const JAVA_10: u16 = 54;
    pub const JAVA_11: u16 = 55;
    pub const JAVA_12: u16 = 56;
    pub const JAVA_14: u16 = 58;
    pub const JAVA_15: u16 = 59;
}
