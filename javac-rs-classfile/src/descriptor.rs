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
    fn array_of(mut component: TypeDescriptor, size: usize) -> Self {
        for _ in 0..size { component = Self::Array(Box::new(component)) }
        component
    }
}

impl ToString for TypeDescriptor {
    fn to_string(&self) -> String {
        match self {
            Self::Byte => 'B'.to_string(),
            Self::Char => 'C'.to_string(),
            Self::Double => 'D'.to_string(),
            Self::Float => 'F'.to_string(),
            Self::Int => 'I'.to_string(),
            Self::Long => 'J'.to_string(),
            Self::Short => 'S'.to_string(),
            Self::Boolean => 'Z'.to_string(),
            Self::Class(name) => format!("L{};", name).to_string(),
            Self::Array(body) => format!("[{}", body.to_string()),
        }
    }
}

pub type FieldDescriptor = TypeDescriptor;

#[allow(clippy::panic)]
#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Class("other/test/ClassName".to_string()))))).to_string(), "[[Lother/test/ClassName;");
    }

    #[test]
    fn array_1_descriptor_factory() {
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Byte, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Byte)), "[B"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Char, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Char)), "[C"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Double, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Double)), "[D"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Float, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Float)), "[F"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Int, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Int)), "[I"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Long, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Long)), "[J"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Short, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Short)), "[S"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Boolean, 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Boolean)), "[Z"
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Class("some/test/ClassName".to_string()), 1),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Class("some/test/ClassName".to_string()))),
        );
    }

    #[test]
    fn array_2_descriptor_factory() {
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Byte, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Byte))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Char, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Char))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Double, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Double))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Float, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Float))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Int, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Int))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Long, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Long))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Short, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Short))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Boolean, 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Boolean))))
        );
        assert_eq!(
            TypeDescriptor::array_of(TypeDescriptor::Class("some/test/ClassName".to_string()), 2),
            TypeDescriptor::Array(Box::new(TypeDescriptor::Array(Box::new(TypeDescriptor::Class("some/test/ClassName".to_string())))))
        );
    }
}
