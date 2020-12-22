use javac_rs_classfile::{Class, ClassfileVersion, major_versions, ClassfileWritable, MethodAccessFlag, ClassAccessFlag, ConstValue, JvmVecU4};
use std::fs::File;
use std::convert::TryFrom;

#[test]
fn class_file_with_method_without_attributes() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithMethodWithoutAttributes"),
        String::from("java/lang/Object"),
    );
    class.add_interface(String::from("java/io/Serializable")).unwrap();
    class.add_method(
        MethodAccessFlag::Public | MethodAccessFlag::Final,
        String::from("foo"),
        String::from("()I")
    ).unwrap();

    let mut file = File::create("TestClassWithMethodWithoutAttributes.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}

#[test]
fn class_file_with_method_with_various_attributes() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithVariousAttributes"),
        String::from("java/lang/Object"),
    );
    class.add_interface(String::from("java/io/Serializable")).unwrap();
    {
        let method = class.add_method(
            MethodAccessFlag::Public | MethodAccessFlag::Static,
            String::from("bar"),
            String::from("(Ljava/lang/String;)Z")
        ).unwrap();
        class.method_add_deprecated_attribute(method);
        class.method_add_synthetic_attribute(method);
        class.method_add_custom_attribute(
            method,
            String::from("\\_MagicalMethodAttribute_/"),
            JvmVecU4::try_from(Vec::from("Oh hi magic".as_bytes())).unwrap(),
        ).unwrap();
        class.method_add_custom_attribute(
            method,
            String::from("////Wow, so much slashes"),
            JvmVecU4::try_from(Vec::from("Yes of course".as_bytes())).unwrap(),
        ).unwrap();
    }

    let mut file = File::create("TestClassWithVariousAttributes.class").unwrap();
    println!("{:#?}", class);
    class.write_to_classfile(&mut file);
    println!("Written to file: {:#?}", file);
}