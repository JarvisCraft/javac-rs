use javac_rs_classfile::*;
use std::fs::File;

mod class_testing;

#[test]
fn class_file() {
    let class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClass"),
        String::from("java/lang/Object"),
    );

    let mut file = File::create("ru/progrm_jarvis/javacrs/TestClass.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
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

    class_testing::dump_class(class, "ru/progrm_jarvis/javacrs/TestClassWithSingleInterface");
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

    let mut file = File::create("ru/progrm_jarvis/javacrs/TestClassWithMultipleInterfaces.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}
