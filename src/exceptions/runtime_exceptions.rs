use crate::values::get_type::GetType;
use crate::values::values::JexValue;
use extendable_vm::{Exception, ExceptionType};

#[derive(Debug)]
pub struct TypeException(pub String);

impl From<TypeException> for Exception {
    fn from(exception: TypeException) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "TypeException".to_string(),
            message: exception.0,
        }
    }
}

#[derive(Debug)]
pub struct ExpectedInstructionArgument;

impl From<ExpectedInstructionArgument> for Exception {
    fn from(_exception: ExpectedInstructionArgument) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "ExpectedInstructionArgument".to_string(),
            message: "Expected an instruction argument but chunk code ended.".to_string(),
        }
    }
}

pub struct UnaryOperatorNotDefined {
    operator_name: &'static str,
    operand_type: String,
}

impl UnaryOperatorNotDefined {
    pub fn new(operator_name: &'static str, operand: &JexValue) -> UnaryOperatorNotDefined {
        UnaryOperatorNotDefined {
            operator_name,
            operand_type: operand.get_type(),
        }
    }
}

impl From<UnaryOperatorNotDefined> for Exception {
    fn from(e: UnaryOperatorNotDefined) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "UnaryOperatorNotDefined".to_string(),
            message: format!(
                "Unary operator {} is not defined for type {}",
                e.operator_name, e.operand_type
            ),
        }
    }
}

pub struct OperatorNotDefined {
    operator_name: &'static str,
    left_type: String,
    right_type: String,
}

impl OperatorNotDefined {
    pub fn new(
        operator_name: &'static str,
        left: &JexValue,
        right: &JexValue,
    ) -> OperatorNotDefined {
        OperatorNotDefined {
            operator_name,
            left_type: left.get_type(),
            right_type: right.get_type(),
        }
    }
}

impl From<OperatorNotDefined> for Exception {
    fn from(e: OperatorNotDefined) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "OperatorUndefined".to_string(),
            message: format!(
                "Binary operator {} is not defined for types {} and {}",
                e.operator_name, e.left_type, e.right_type
            ),
        }
    }
}

#[derive(Debug)]
pub struct FieldNotFound(pub String);

impl From<FieldNotFound> for Exception {
    fn from(exception: FieldNotFound) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "FieldNotFound".to_string(),
            message: format!("Field {} of object not found", exception.0),
        }
    }
}


