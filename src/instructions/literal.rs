use crate::instructions::types::JexInstruction;
use extendable_vm::{Instruction, InstructionPointer, Exception, ByteReadable, InstructionFn};
use crate::instructions::op_codes::JexOpCode;
use crate::types::JexMachine;
use crate::runtime_exceptions::ExpectedInstructionArgument;

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
