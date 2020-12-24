use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::Path;
use std::process::{Command, Output};

use javac_rs_classfile::{Class, ClassfileWritable};

// TODO consider using result instead of panicing

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct ClassTester {
    class_name: String,
}

impl ClassTester {
    pub fn run(&self) -> Result<Output> {
        Ok(Command::new("java").arg(&self.class_name).output()?)
    }

    pub fn run_with_args<I: IntoIterator<Item=S>, S: AsRef<OsStr>>(
        &self,
        args: I,
    ) -> io::Result<Output> {
        Command::new("java")
            .args(args)
            .arg(&self.class_name)
            .output()
    }

    pub fn disasm(&self) -> io::Result<Output> {
        Command::new("javap").arg(&self.class_name).output()
    }

    pub fn disasm_with_args<I: IntoIterator<Item=S>, S: AsRef<OsStr>>(
        &self,
        args: I,
    ) -> io::Result<Output> {
        Command::new("javap")
            .args(args)
            .arg(&self.class_name)
            .output()
    }

    pub fn assert_disasmable(&self) {
        let result = self
            .disasm()
            .expect(&format!("Cannot execute javap for {}", self.class_name));
        assert_eq!(
            result
                .status
                .code()
                .expect("Class disassembly was terminated by a signal"),
            0,
            "Class could not be disassembled{}",
            String::from_utf8(result.stdout).map_or("".to_string(), |error| {
                let mut out = ": ".to_string();
                out.push_str(&error);
                out
            })
        )
    }
}

pub fn dump_class(class: Class, class_name: String) -> Result<ClassTester> {
    let mut path = class_name.clone().replace('.', "/");
    path.push_str(".class");
    let path = Path::new(&path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;

    class.write_to_classfile(&mut file);

    Ok(ClassTester { class_name })
}
