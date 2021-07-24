use extendable_vm::{Code, Machine, InstructionTable};
use crate::bytecode_constants::JexConstant;
use crate::types::JexMachine;
use crate::instructions::JEX_INSTRUCTIONS;
use crate::jex_values::values::{JexFunction, JexValue};

pub mod operators;
pub mod runtime_exceptions;
pub mod syntax_exceptions;
pub mod constant_parsers;
pub mod bytecode_constants;
pub mod jex_values;
pub mod instructions;

pub fn build_jex_machine(code: &Code<JexConstant>) -> JexMachine {
    let instruction_table = InstructionTable::instructions(&JEX_INSTRUCTIONS);
    let mut machine = Machine::new(code, instruction_table);
    machine.push_operand(JexValue::Function(JexFunction::Script));
    machine.push_frame(0, "<script>".to_string(), 0);
    machine
}

pub mod types {
    use extendable_vm::Machine;
    use crate::bytecode_constants::JexConstant;
    use crate::jex_values::values::JexValue;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}