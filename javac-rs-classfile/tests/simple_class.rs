use std::fs::File;

use javac_rs_classfile::*;

mod class_testing;

#[test]
fn class_file() {
    let class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClass"),
        String::from("java/lang/Object"),
    );

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClass".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}

#[test]
fn class_file_with_single_interface() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithSingleInterface"),
        String::from("java/lang/Object"),
    );
    class.add_interface(String::from("java/io/Serializable"));
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithSingleInterface".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}

#[test]
fn class_file_with_multiple_interface() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithMultipleInterfaces"),
        String::from("java/lang/Object"),
    );
    class.add_interface(String::from("java/lang/Cloneable"));
    class.add_interface(String::from("java/io/Serializable"));
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithMultipleInterfaces".to_string(),
    )
        .unwrap()
        .assert_disasmable();
}
