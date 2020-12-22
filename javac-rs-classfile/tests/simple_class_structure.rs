use javac_rs_classfile::*;
use std::fs::File;
use std::io::Write;

#[test]
fn create_minimal_class_file() {
    let class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClass"),
        String::from("java/lang/Object"),
    );
    let class = class;

    let mut file = File::create("TestClass.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}

#[test]
fn create_minimal_class_file_with_interfaces() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithInterfaces"),
        String::from("java/lang/Object"),
    );
    class.add_interface(String::from("java/io/Serializable"));
    let class = class;

    let mut file = File::create("TestClassWithInterfaces.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}
