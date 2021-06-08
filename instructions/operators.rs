use crate::jex::instructions::op_codes::JexOpCode;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::jex_values::to_output_string::ToOutputString;
use crate::jex::jex_values::values::{JexNull, JexObject, JexValue};
use crate::jex::operators::{
    divide, equal, greater, less, minus, multiply, negate, not, plus, print,
};
use crate::jex::runtime_exceptions::TypeException;
use crate::jex::types::JexMachine;
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction::Instruction;
use crate::machine::instruction::InstructionFn::{BinaryOp, Const, UnaryOp};
use crate::machine::instruction_pointer::InstructionPointer;
use std::rc::Rc;

pub const NEGATE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Negate as u8,
    name: "NEGATE",
    instruction_fn: UnaryOp(negate),
};

pub const ADD_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Add as u8,
    name: "ADD",
    instruction_fn: BinaryOp(plus),
};

pub const SUBTRACT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Subtract as u8,
    name: "SUBTRACT",
    instruction_fn: BinaryOp(minus),
};

pub const MULTIPLY_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Multiply as u8,
    name: "MULTIPLY",
    instruction_fn: BinaryOp(multiply),
};

pub const DIVIDE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Divide as u8,
    name: "DIVIDE",
    instruction_fn: BinaryOp(divide),
};

pub const EQUAL_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Equal as u8,
    name: "EQUAL",
    instruction_fn: BinaryOp(equal),
};

pub const GREATER_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Greater as u8,
    name: "GREATER",
    instruction_fn: BinaryOp(greater),
};

pub const LESS_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Less as u8,
    name: "LESS",
    instruction_fn: BinaryOp(less),
};

pub const NOT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Not as u8,
    name: "NOT",
    instruction_fn: UnaryOp(not),
};

pub const PRINT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Print as u8,
    name: "PRINT",
    instruction_fn: UnaryOp(print),
};

pub const NULL_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Null as u8,
    name: "NULL",
    instruction_fn: Const(|| JexValue::null()),
};

pub const TRUE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::True as u8,
    name: "TRUE",
    instruction_fn: Const(|| JexValue::Bool(true)),
};

pub const FALSE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::False as u8,
    name: "FALSE",
    instruction_fn: Const(|| JexValue::Bool(false)),
};