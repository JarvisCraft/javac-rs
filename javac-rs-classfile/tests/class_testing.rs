use javac_rs_classfile::{Class, ClassfileWritable};
use std::fs::File;
use std::path::Path;

pub fn dump_class<P: AsRef<Path>>(class: Class, file: P) {
    let path = file.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .expect("Could not create parent directory");
    }
    let mut file = File::create(path)
        .expect("Could not create file from path");

    println!("Writing class:\n{:#?}\nto file{:#?}", class, file);
    class.write_to_classfile(&mut file);
}
