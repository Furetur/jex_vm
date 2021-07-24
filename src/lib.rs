use extendable_vm::{Code, InstructionTable, Machine};

use code::bytecode_constants::JexConstant;

use crate::instructions::JEX_INSTRUCTIONS;
use crate::types::JexMachine;
use crate::values::values::{JexFunction, JexValue};

pub mod instructions;
pub mod values;
pub mod code;
pub mod exceptions;

pub fn build_jex_machine(code: &Code<JexConstant>) -> JexMachine {
    let instruction_table = InstructionTable::instructions(&JEX_INSTRUCTIONS);
    let mut machine = Machine::new(code, instruction_table);
    machine.push_operand(JexValue::Function(JexFunction::Script));
    machine.push_frame(0, "<script>".to_string(), 0);
    machine
}

pub mod types {
    use extendable_vm::Machine;

    use crate::code::bytecode_constants::JexConstant;
    use crate::values::values::JexValue;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}
