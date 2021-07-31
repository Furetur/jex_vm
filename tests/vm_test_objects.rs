use jex_vm::code::bytecode_constants::JexConstant;
use jex_vm::instructions::op_codes::JexOpCode;
use jex_vm::values::values::JexValue;
use run::code::{TestChunk, TestInstruction};
use run::run_jex::run_chunk;

mod run;

#[test]
fn it_should_create_empty_object() {
    let result = run_chunk(TestChunk {
        constants: vec![],
        instructions: vec![TestInstruction {
            op_code: JexOpCode::NewInstance,
            args: vec![],
        }],
    });

    assert!(result.unwrap().as_instance().unwrap().is_empty())
}

#[test]
fn it_should_set_field_of_empty_object() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::String("field_name".to_string())],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::NewInstance,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::True,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::SetField,
                args: vec![0],
            },
        ],
    });
    let result = result.unwrap();
    let instance = result.as_instance().unwrap();
    assert!(!instance.is_empty());
    assert_eq!(
        JexValue::Bool(true),
        instance.get_field("field_name").unwrap()
    );
}

#[test]
fn it_should_get_field_after_setting_field() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::String("field_name".to_string())],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::NewInstance,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::True,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::SetField,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::GetField,
                args: vec![0],
            },
        ],
    });
    let result = result.unwrap();
    let bool = result.as_bool().unwrap();
    assert!(bool)
}

// exceptions

#[test]
#[should_panic]
fn it_should_panic_if_get_field_of_empty_object() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::String("field_name".to_string())],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::NewInstance,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::GetField,
                args: vec![0],
            },
        ],
    });
}

#[test]
#[should_panic]
fn it_should_panic_if_get_field_of_bool() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::String("field_name".to_string())],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::True,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::GetField,
                args: vec![0],
            },
        ],
    });
}

#[test]
#[should_panic]
fn it_should_panic_if_set_field_of_bool() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::String("field_name".to_string())],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::True,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::False,
                args: vec![],
            },
            TestInstruction {
                op_code: JexOpCode::SetField,
                args: vec![0],
            },
        ],
    });
}
