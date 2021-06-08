use crate::jex::instructions::types::JexInstruction;
use crate::jex::jex_values::to_output_string::ToOutputString;
use crate::jex::jex_values::values::JexValue;
use crate::jex::types::JexMachine;
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::Instruction;

pub fn side_effects_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut side_effects_instructions = vec![Instruction {
        op_code: 10,
        name: "PRINT".to_string(),
        byte_arity: 0,
        instruction_fn: print_instruction,
    }];
    instructions.append(&mut side_effects_instructions);
}

fn print_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), Exception> {
    let value = machine.pop_operand()?;
    machine.push_operand(JexValue::null());
    println!("{}", value.to_output_string());
    Ok(())
}
