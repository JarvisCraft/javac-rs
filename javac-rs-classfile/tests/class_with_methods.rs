use std::convert::TryFrom;

use javac_rs_classfile::{
    Bytecode, Class, ClassAccessFlag, ClassfileVersion, JvmVecU4, major_versions, MethodAccessFlag,
};

mod class_testing;

#[test]
fn class_file_with_method_without_attributes() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithMethodWithoutAttributes"),
        String::from("java/lang/Object"),
    );
    class
        .add_interface(String::from("java/io/Serializable"))
        .unwrap();
    class
        .add_method(
            MethodAccessFlag::Public | MethodAccessFlag::Final,
            String::from("foo"),
            String::from("()I"),
        )
        .unwrap();

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithMethodWithoutAttributes".to_string(),
    ).unwrap().run();
}

#[test]
fn class_file_with_method_with_various_attributes() {
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
        let method = class
            .add_method(
                MethodAccessFlag::Public | MethodAccessFlag::Static,
                String::from("bar"),
                String::from("(Ljava/lang/String;)Z"),
            )
            .unwrap();
        class.method_add_deprecated_attribute(method);
        class.method_add_synthetic_attribute(method);
        class
            .method_add_custom_attribute(
                method,
                String::from("\\_MagicalMethodAttribute_/"),
                JvmVecU4::try_from(Vec::from("Oh hi magic".as_bytes())).unwrap(),
            )
            .unwrap();
        class
            .method_add_custom_attribute(
                method,
                String::from("////Wow, so much slashes"),
                JvmVecU4::try_from(Vec::from("Yes of course".as_bytes())).unwrap(),
            )
            .unwrap();
    }
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithVariousAttributes".to_string(),
    ).unwrap().assert_disasmable();
}

#[test]
fn class_file_with_method_with_code_attribute() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/TestClassWithCodeAttribute"),
        String::from("java/lang/Object"),
    );
    class
        .add_interface(String::from("java/io/Serializable"))
        .unwrap();
    {
        let method = class
            .add_method(
                MethodAccessFlag::Public | MethodAccessFlag::Static,
                String::from("bar"),
                String::from("(Ljava/lang/String;)Ljava/lang/String;"),
            )
            .unwrap();
        let mut bytecode = Bytecode::new(1);
        bytecode.instr_aload(0).unwrap();
        bytecode.instr_areturn();
        println!("Bytecode: {:?}", bytecode);
        class.method_add_code_attribute(method, bytecode);
    }
    let class = class;

    class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.TestClassWithCodeAttribute".to_string(),
    ).unwrap().assert_disasmable();
}

#[test]
fn class_file_with_hello_world_main_method() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/ClassWithHelloWorldMainMethod"),
        String::from("java/lang/Object"),
    );
    class
        .add_interface(String::from("java/io/Serializable"))
        .unwrap();
    {
        let method = class
            .add_method(
                MethodAccessFlag::Public | MethodAccessFlag::Static,
                String::from("main"),
                String::from("([Ljava/lang/String;)V"),
            )
            .unwrap();
        let mut bytecode = Bytecode::new(1);
        let mut const_pool = class.const_pool_mut();

        // getstatic System.out
        bytecode
            .instr_getstatic(
                const_pool
                    .store_const_field_ref_info(
                        "java/lang/System".to_string(),
                        "out".to_string(),
                        "Ljava/io/PrintStream;".to_string(),
                    )
                    .unwrap(),
                false,
            )
            .unwrap();

        // ldc "HelloWorld"
        bytecode
            .instr_ldc(
                const_pool
                    .store_const_string_info("Hello world!".to_string())
                    .unwrap(),
            )
            .unwrap();

        // invokevirtual PrintStream#println(String)
        bytecode
            .instr_invokevirtual(
                const_pool
                    .store_const_method_ref_info(
                        "java/io/PrintStream".to_string(),
                        "println".to_string(),
                        "(Ljava/lang/String;)V".to_string(),
                    )
                    .unwrap(),
                1,
            )
            .unwrap();

        // return
        bytecode.instr_return().unwrap();

        class.method_add_code_attribute(method, bytecode).unwrap();
    }
    let class = class;

    let result = class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.ClassWithHelloWorldMainMethod".to_string(),
    )
        .unwrap()
        .run()
        .unwrap();
    assert_eq!(result.status.code().expect("Process terminated with signal"), 0);
    assert_eq!(result.stdout, b"Hello world!\n");
}

// currently stack frame attributes are not set thus the class should be run using `-noverify`
#[test]
fn class_file_with_naive_loop_in_main_method() {
    let mut class = Class::new(
        ClassfileVersion::of_major(major_versions::JAVA_14),
        ClassAccessFlag::Public | ClassAccessFlag::Final | ClassAccessFlag::Super,
        String::from("ru/progrm_jarvis/javacrs/ClassWithNaiveLoopInMainMethod"),
        String::from("java/lang/Object"),
    );
    class
        .add_interface(String::from("java/io/Serializable"))
        .unwrap();
    {
        let method = class
            .add_method(
                MethodAccessFlag::Public | MethodAccessFlag::Static,
                String::from("main"),
                String::from("([Ljava/lang/String;)V"),
            )
            .unwrap();
        let mut bytecode = Bytecode::new(2);
        let mut const_pool = class.const_pool_mut();

        // bipush 6
        bytecode.instr_bipush(6);
        // istore 1
        bytecode.instr_istore(1);

        // iinc 1 -1
        let loop_head = bytecode.instr_iinc(1, -1).unwrap();

        // getstatic System.out
        bytecode
            .instr_getstatic(
                const_pool
                    .store_const_field_ref_info(
                        "java/lang/System".to_string(),
                        "out".to_string(),
                        "Ljava/io/PrintStream;".to_string(),
                    )
                    .unwrap(),
                false,
            )
            .unwrap();

        // iload 1
        bytecode.instr_iload(1).unwrap();
        // dup_x1
        bytecode.instr_dup_x1().unwrap();
        // invokevirtual PrintStream#println(int)
        bytecode
            .instr_invokevirtual(
                const_pool
                    .store_const_method_ref_info(
                        "java/io/PrintStream".to_string(),
                        "println".to_string(),
                        "(I)V".to_string(),
                    )
                    .unwrap(),
                1,
            )
            .unwrap();

        // ifne #loop_head
        bytecode.instr_ifne(loop_head);

        // return
        bytecode.instr_return().unwrap();

        class.method_add_code_attribute(method, bytecode).unwrap();
    }
    let class = class;

    let result = class_testing::dump_class(
        class,
        "ru.progrm_jarvis.javacrs.ClassWithNaiveLoopInMainMethod".to_string(),
    )
        .unwrap()
        .run_with_args(&["-noverify"])
        .unwrap();
    assert_eq!(
        result
            .status
            .code()
            .expect("Process terminated with signal"),
        0
    );
    assert_eq!(
        result.stdout,
        b"5\n\
        4\n\
        3\n\
        2\n\
        1\n\
        0\n\
        "
    );
}
