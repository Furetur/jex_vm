use crate::exceptions::runtime_exceptions::TypeException;
use crate::types::JexMachine;
use crate::values::values::{JexFunction, JexValue};
use extendable_vm::Exception;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JexConstant {
    Int(i32),
    String(String),
    Function { chunk_id: usize },
}

pub enum JexConstantType {
    Int = 0,
    String = 1,
    Function = 2,
}

impl JexConstant {
    pub fn to_value(&self, machine: &JexMachine) -> Result<JexValue, Exception> {
        let value = match self {
            JexConstant::Int(i) => JexValue::Int(*i),
            JexConstant::String(str) => JexValue::from_string(str.clone()),
            JexConstant::Function { chunk_id } => {
                let func = JexFunction::from_code(machine, *chunk_id)?;
                JexValue::Function(func)
            }
        };
        Ok(value)
    }
    pub fn from_str(str: &str) -> JexConstant {
        JexConstant::String(str.to_string())
    }
    pub fn as_string(&self) -> Result<String, TypeException> {
        if let JexConstant::String(string) = self {
            Ok(string.clone())
        } else {
            Err(TypeException(format!(
                "Expected chunk constant {:?} to be string",
                self
            )))
        }
    }
    pub fn as_int(&self) -> Result<i32, TypeException> {
        if let JexConstant::Int(int) = self {
            Ok(*int)
        } else {
            Err(TypeException(format!(
                "Expected chunk constant {:?} to be int",
                self
            )))
        }
    }
}
