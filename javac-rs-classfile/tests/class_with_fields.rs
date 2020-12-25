use std::convert::TryFrom;
use std::fs::File;

use javac_rs_classfile::{
    major_versions, Class, ClassAccessFlag, ClassfileVersion, ClassfileWritable, ConstValue,
    FieldAccessFlag, FieldDescriptor, JvmVecU4,
};

mod class_testing;

#[test]
fn class_file_with_field_without_attributes() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithFieldWithoutAttributes"),
        String::from("java/lang/Object"),
    );
    class
        .add_interface(String::from("java/io/Serializable"))
        .unwrap();
    class
        .add_field(
            FieldAccessFlag::Private | FieldAccessFlag::Static,
            String::from("val"),
            FieldDescriptor::Int,
        )
        .unwrap();
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithFieldWithoutAttributes".to_string(),
    )
    .unwrap()
    .assert_disasmable();
}

#[test]
fn class_file_with_field_with_const_value_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithConstValueAttribute"),
        String::from("java/lang/Object"),
    );
    class
        .add_interface(String::from("java/io/Serializable"))
        .unwrap();
    {
        let field = class
            .add_field(
                FieldAccessFlag::Public | FieldAccessFlag::Static,
                String::from("val"),
                FieldDescriptor::Int,
            )
            .unwrap();
        class.field_add_const_value_attribute(field, ConstValue::Integer(123));
    }
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithConstValueAttribute".to_string(),
    )
    .unwrap()
    .assert_disasmable();
}

#[test]
fn class_file_with_field_with_various_attributes() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithVariousAttributes"),
        String::from("java/lang/Object"),
    );
    class
        .add_interface(String::from("java/io/Serializable"))
        .unwrap();
    {
        let field = class
            .add_field(
                FieldAccessFlag::Public | FieldAccessFlag::Static,
                String::from("val"),
                FieldDescriptor::Int,
            )
            .unwrap();
        class.field_add_const_value_attribute(field, ConstValue::Integer(123));
        class.field_add_deprecated_attribute(field);
        class.field_add_synthetic_attribute(field);
        class
            .field_add_custom_attribute(
                field,
                String::from("\\_MagicalFieldAttribute_/"),
                JvmVecU4::try_from(Vec::from("Oh hi magic".as_bytes())).unwrap(),
            )
            .unwrap();
        class
            .field_add_custom_attribute(
                field,
                String::from("////Wow, so much slashes"),
                JvmVecU4::try_from(Vec::from("Yes of course".as_bytes())).unwrap(),
            )
            .unwrap();
    }
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithVariousAttributes".to_string(),
    )
    .unwrap()
    .assert_disasmable();
}
