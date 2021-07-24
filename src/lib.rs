use crate::bytecode_constants::JexConstant;
use crate::instructions::JEX_INSTRUCTIONS;
use crate::jex_values::values::{JexFunction, JexValue};
use crate::types::JexMachine;
use extendable_vm::{Code, InstructionTable, Machine};

pub mod bytecode_constants;
pub mod constant_parsers;
pub mod instructions;
pub mod jex_values;
pub mod operators;
pub mod runtime_exceptions;
pub mod syntax_exceptions;

pub fn build_jex_machine(code: &Code<JexConstant>) -> JexMachine {
    let instruction_table = InstructionTable::instructions(&JEX_INSTRUCTIONS);
    let mut machine = Machine::new(code, instruction_table);
    machine.push_operand(JexValue::Function(JexFunction::Script));
    machine.push_frame(0, "<script>".to_string(), 0);
    machine
}

pub mod types {
    use crate::bytecode_constants::JexConstant;
    use crate::jex_values::values::JexValue;
    use extendable_vm::Machine;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}
