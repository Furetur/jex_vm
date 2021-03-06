use crate::exceptions::runtime_exceptions::ExpectedInstructionArgument;
use crate::instructions::op_codes::JexOpCode;
use crate::instructions::types::JexInstruction;
use crate::types::JexMachine;
use extendable_vm::{ByteReadable, Exception, Instruction, InstructionFn, InstructionPointer};

pub const CONSTANT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Constant as u8,
    name: "CONSTANT",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 1,
        instruction_fn: constant_instruction,
    },
};

fn constant_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let constant_id = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let constant = machine
        .code
        .get_constant(args.chunk_id, usize::from(constant_id))?;
    machine.push_operand(constant.to_value(machine)?);
    Ok(())
}
