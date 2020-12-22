use javac_rs_classfile::*;
use std::fs::File;
use std::convert::TryFrom;

#[test]
fn create_minimal_class_file() {
    let class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClass"),
        String::from("java/lang/Object"),
    );

    let mut file = File::create("TestClass.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}

#[test]
fn create_minimal_class_file_with_single_interface() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithSingleInterface"),
        String::from("java/lang/Object"),
    );
    class.add_interface(String::from("java/io/Serializable"));
    let class = class;

    let mut file = File::create("TestClassWithSingleInterface.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}

#[test]
fn create_minimal_class_file_with_multiple_interface() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithMultipleInterfaces"),
        String::from("java/lang/Object"),
    );
    class.add_interface(String::from("java/lang/Cloneable"));
    class.add_interface(String::from("java/io/Serializable"));
    let class = class;

    let mut file = File::create("TestClassWithMultipleInterfaces.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}

#[test]
fn create_minimal_class_file_with_single_custom_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithSingleCustomAttribute"),
        String::from("java/lang/Object"),
    );
    class.add_custom_attribute(
        String::from("SomeCustomAttribute"),
        JvmVecU4::try_from(Vec::from("Hello world".as_bytes())).unwrap(),
    ).unwrap();
    let class = class;

    let mut file = File::create("TestClassWithSingleCustomAttribute.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}

#[test]
fn create_minimal_class_file_with_multiple_custom_attributes() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithMultipleCustomAttributes"),
        String::from("java/lang/Object"),
    );
    class.add_custom_attribute(
        String::from("SomeCustomAttribute"),
        JvmVecU4::try_from(Vec::from("Hello world".as_bytes())).unwrap(),
    ).unwrap();
    class.add_custom_attribute(
        String::from("OtherCustomAttribute"),
        JvmVecU4::try_from(Vec::from("How r u?".as_bytes())).unwrap(),
    ).unwrap();
    let class = class;

    let mut file = File::create("TestClassWithMultipleCustomAttributes.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}
