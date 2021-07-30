use crate::exceptions::runtime_exceptions::{
    ExpectedInstructionArgument, FieldNotFound, UnaryOperatorNotDefined,
};
use crate::instructions::op_codes::JexOpCode;
use crate::instructions::types::JexInstruction;
use crate::types::JexMachine;
use crate::values::values::JexValue;
use extendable_vm::{ByteReadable, Exception, Instruction, InstructionFn, InstructionPointer};

pub const NEW_OBJECT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::NewObject as u8,
    name: "NEW_OBJECT",
    instruction_fn: InstructionFn::Const(|| JexValue::new_object()),
};

pub const GET_FIELD_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::GetField as u8,
    name: "GET_FIELD",
    instruction_fn: InstructionFn::Raw {
        byte_arity: 1,
        instruction_fn: get_field_instruction,
    },
};

fn get_field_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    // read constant_id of field name
    let constant_id = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let field_name = machine
        .code
        .get_constant(args.chunk_id, usize::from(constant_id))?
        .as_string()?;
    let instance = machine.peek_operand()?;
    if let JexValue::Instance(instance) = instance {
        let field = instance
            .get_field(field_name.as_str())
            .ok_or(FieldNotFound(field_name))?;
        machine.push_operand(field);
        Ok(())
    } else {
        Err(Exception::from(UnaryOperatorNotDefined::new(
            "GET_FIELD",
            instance,
        )))
    }
}
