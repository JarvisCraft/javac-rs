use std::convert::TryFrom;
use std::fs::File;

use javac_rs_classfile::*;

mod class_testing;

#[test]
fn class_file_with_source_file_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithSourceFileAttribute"),
        String::from("java/lang/Object"),
    );
    class
        .add_source_file_attribute(String::from(
            "ru/progrm_jarvis/javacrs/TestClassWithSourceFileAttribute.java",
        ))
        .unwrap();
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithSourceFileAttribute".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}

#[test]
fn class_file_with_synthetic_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithSyntheticAttribute"),
        String::from("java/lang/Object"),
    );
    class.add_synthetic_attribute().unwrap();
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithSyntheticAttribute".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}

#[test]
fn class_file_with_deprecated_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithDeprecatedAttribute"),
        String::from("java/lang/Object"),
    );
    class.add_deprecated_attribute().unwrap();
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithDeprecatedAttribute".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}

#[test]
fn class_file_with_signature_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithSignatureAttribute"),
        String::from("java/lang/Object"),
    );
    class
        .add_signature_attribute(String::from("<T:Ljava/lang/Object;>Ljava/lang/Object;"))
        .unwrap();
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithSignatureAttribute".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}

#[test]
fn class_file_with_single_custom_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithSingleCustomAttribute"),
        String::from("java/lang/Object"),
    );
    class
        .add_custom_attribute(
            String::from("SomeCustomAttribute"),
            JvmVecU4::try_from(Vec::from("Hello world".as_bytes())).unwrap(),
        )
        .unwrap();
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithSingleCustomAttribute".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}

#[test]
fn class_file_with_multiple_custom_attributes() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithMultipleCustomAttributes"),
        String::from("java/lang/Object"),
    );
    class
        .add_custom_attribute(
            String::from("SomeCustomAttribute"),
            JvmVecU4::try_from(Vec::from("Hello world".as_bytes())).unwrap(),
        )
        .unwrap();
    class
        .add_custom_attribute(
            String::from("OtherCustomAttribute"),
            JvmVecU4::try_from(Vec::from("How r u?".as_bytes())).unwrap(),
        )
        .unwrap();
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithMultipleCustomAttributes".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}
