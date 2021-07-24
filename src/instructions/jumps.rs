use crate::exceptions::runtime_exceptions::{ExpectedInstructionArgument, TypeException};
use crate::instructions::op_codes::JexOpCode;
use crate::instructions::types::JexInstruction;
use crate::types::JexMachine;
use crate::values::to_output_string::ToOutputString;
use crate::values::values::JexFunction;
use extendable_vm::{ByteReadable, Exception, Instruction, InstructionFn, InstructionPointer};

pub const JUMP_FORWARD_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::JumpForward as u8,
    name: "JUMP_FORWARD",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 1,
        instruction_fn: jump_forward_instruction,
    },
};

pub const JUMP_FORWARD_IF_FALSE_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::JumpForwardIfFalse as u8,
    name: "JUMP_FORWARD_IF_FALSE",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 1,
        instruction_fn: jump_forward_if_false_instruction,
    },
};

pub const JUMP_BACKWARD: JexInstruction = Instruction {
    op_code: JexOpCode::JumpBackward as u8,
    name: "JUMP_BACKWARD",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 1,
        instruction_fn: jump_backward_instruction,
    },
};

pub const CALL_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Call as u8,
    name: "CALL",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 1,
        instruction_fn: call_instruction,
    },
};

pub const RETURN_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Return as u8,
    name: "RETURN",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 0,
        instruction_fn: return_instruction,
    },
};

fn jump_forward_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let offset = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    machine
        .instruction_pointer()?
        .jump_forward(usize::from(offset));
    Ok(())
}

fn jump_forward_if_false_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let offset = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let value = machine
        .peek_operand()?
        .as_bool()
        .ok_or_else(|| TypeException("Jump forward if false condition was not bool".to_string()))?;
    if !value {
        machine
            .instruction_pointer()?
            .jump_forward(usize::from(offset));
    }
    Ok(())
}

fn jump_backward_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let offset = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    machine
        .instruction_pointer()?
        .jump_backward(usize::from(offset));
    Ok(())
}

fn call_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let arity = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let arity = usize::from(arity);
    let function = machine
        .get_operand_from_top(arity)?
        .as_function()
        .ok_or_else(|| TypeException("Value was not callable".to_string()))?;
    if let JexFunction::Function {
        chunk_id,
        arity: actual_arity,
        name,
    } = function
    {
        if arity == *actual_arity {
            let chunk_start_slot = machine.operand_stack_len() - 1 - arity;
            machine.push_frame(*chunk_id, name.clone(), chunk_start_slot);
            Ok(())
        } else {
            Err(Exception::from(TypeException(format!(
                "Function {} has {} parameters but received {}",
                function.to_output_string(),
                actual_arity,
                arity
            ))))
        }
    } else {
        Err(Exception::from(TypeException(format!(
            "Cannot call {}",
            function.to_output_string()
        ))))
    }
}

fn return_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), Exception> {
    let return_value = machine.pop_operand()?;
    machine.discard_frame()?;
    machine.push_operand(return_value);
    Ok(())
}
