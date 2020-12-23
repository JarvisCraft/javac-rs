//! Tools for generating correct JVM descriptors as specified by
//! [#4.3](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.3)

#[derive(Eq, PartialEq, Debug)]
pub enum TypeDescriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Class(String),
    Array(Box<TypeDescriptor>),
}

impl TypeDescriptor {
    //fn array_descriptor//
}

impl ToString for TypeDescriptor {
    fn to_string(&self) -> String {
        match self {
            TypeDescriptor::Byte => 'B'.to_string(),
            TypeDescriptor::Char => 'C'.to_string(),
            TypeDescriptor::Double => 'D'.to_string(),
            TypeDescriptor::Float => 'F'.to_string(),
            TypeDescriptor::Int => 'I'.to_string(),
            TypeDescriptor::Long => 'J'.to_string(),
            TypeDescriptor::Short => 'S'.to_string(),
            TypeDescriptor::Boolean => 'Z'.to_string(),
            TypeDescriptor::Class(name) => format!("L{};", name).to_string(),
            TypeDescriptor::Array(body) => format!("[{}", body.to_string()),
        }
    }
}

pub type FieldDescriptor = TypeDescriptor;

#[allow(clippy::panic)]
#[cfg(test)]
mod tests {
    use crate::TypeDescriptor;

    #[test]
    fn primitive_descriptors() {
        assert_eq!(TypeDescriptor::Byte.to_string(), "B");
        assert_eq!(TypeDescriptor::Char.to_string(), "C");
        assert_eq!(TypeDescriptor::Double.to_string(), "D");
        assert_eq!(TypeDescriptor::Float.to_string(), "F");
        assert_eq!(TypeDescriptor::Int.to_string(), "I");
        assert_eq!(TypeDescriptor::Long.to_string(), "J");
        assert_eq!(TypeDescriptor::Short.to_string(), "S");
        assert_eq!(TypeDescriptor::Boolean.to_string(), "Z");
    }

    #[test]
    fn class_descriptors() {
        assert_eq!(TypeDescriptor::Class("ru/javacrs/TestClass".to_string()).to_string(), "Lru/javacrs/TestClass;");
        assert_eq!(TypeDescriptor::Class("TestClassWithNoPackage".to_string()).to_string(), "LTestClassWithNoPackage;");
        assert_eq!(TypeDescriptor::Class("class.with.Dots".to_string()).to_string(), "Lclass.with.Dots;");
        assert_eq!(TypeDescriptor::Class("int".to_string()).to_string(), "Lint;");
    }

    #[test]
    fn array_1_descriptors() {
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Byte)).to_string(), "[B");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Char)).to_string(), "[C");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Double)).to_string(), "[D");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Float)).to_string(), "[F");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Int)).to_string(), "[I");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Long)).to_string(), "[J");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Short)).to_string(), "[S");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Boolean)).to_string(), "[Z");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Class("some/test/ClassName".to_string()))).to_string(), "[Lsome/test/ClassName;");
    }

    #[test]
    fn array_2_descriptors() {
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Byte)))).to_string(), "[[B");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Char)))).to_string(), "[[C");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Double)))).to_string(), "[[D");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Float)))).to_string(), "[[F");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Int)))).to_string(), "[[I");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Long)))).to_string(), "[[J");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Short)))).to_string(), "[[S");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Boolean)))).to_string(), "[[Z");
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Class("other/test/ClassName".to_string()))).to_string(), "[Lother/test/ClassName;");
    }
}
