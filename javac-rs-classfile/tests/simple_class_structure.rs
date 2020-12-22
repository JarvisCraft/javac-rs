use javac_rs_classfile::*;
use std::fs::File;
use std::io::Write;

#[test]
fn create_simple_class_file() {
    let class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Super | ClassAccessFlag::Final,
        String::from("ru/progrm_jarvis/javacrs/TestClass"),
        String::from("java/lang/Object"),
    );

    let mut buffer: Vec<u8> = Vec::new();

    println!("{:#?}", class);
    class.write_to_classfile(&mut buffer);
    println!("{:?}", buffer);

    let mut file = File::create("TestClass.class").unwrap();
    println!("Path to file: {:#?}", file);
    file.write(&buffer).unwrap();

    // TODO: check class correctness via bundled javap
}
