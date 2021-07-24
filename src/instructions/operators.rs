use crate::instructions::op_codes::JexOpCode;
use crate::instructions::operator_implementations::{
    divide, equal, greater, less, minus, multiply, negate, not, parse_int, plus, print, read_line,
    to_string,
};
use crate::instructions::types::JexInstruction;
use crate::values::values::JexValue;
use extendable_vm::{Instruction, InstructionFn};

pub const NEGATE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Negate as u8,
    name: "NEGATE",
    instruction_fn: InstructionFn::UnaryOp(negate),
};

pub const ADD_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Add as u8,
    name: "ADD",
    instruction_fn: InstructionFn::BinaryOp(plus),
};

pub const SUBTRACT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Subtract as u8,
    name: "SUBTRACT",
    instruction_fn: InstructionFn::BinaryOp(minus),
};

pub const MULTIPLY_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Multiply as u8,
    name: "MULTIPLY",
    instruction_fn: InstructionFn::BinaryOp(multiply),
};

pub const DIVIDE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Divide as u8,
    name: "DIVIDE",
    instruction_fn: InstructionFn::BinaryOp(divide),
};

pub const EQUAL_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Equal as u8,
    name: "EQUAL",
    instruction_fn: InstructionFn::BinaryOp(equal),
};

pub const GREATER_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Greater as u8,
    name: "GREATER",
    instruction_fn: InstructionFn::BinaryOp(greater),
};

pub const LESS_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Less as u8,
    name: "LESS",
    instruction_fn: InstructionFn::BinaryOp(less),
};

pub const NOT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Not as u8,
    name: "NOT",
    instruction_fn: InstructionFn::UnaryOp(not),
};

pub const TO_STRING_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::ToString as u8,
    name: "TO_STRING",
    instruction_fn: InstructionFn::UnaryOp(to_string),
};

pub const PRINT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Print as u8,
    name: "PRINT",
    instruction_fn: InstructionFn::UnaryOp(print),
};

pub const READ_LINE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::ReadLine as u8,
    name: "READ_LINE",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 0,
        instruction_fn: read_line,
    },
};

pub const PARSE_INT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::ParseInt as u8,
    name: "PARSE_INT",
    instruction_fn: InstructionFn::UnaryOp(parse_int),
};

pub const NULL_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Null as u8,
    name: "NULL",
    instruction_fn: InstructionFn::Const(JexValue::null),
};

pub const TRUE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::True as u8,
    name: "TRUE",
    instruction_fn: InstructionFn::Const(|| JexValue::Bool(true)),
};

pub const FALSE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::False as u8,
    name: "FALSE",
    instruction_fn: InstructionFn::Const(|| JexValue::Bool(false)),
};
