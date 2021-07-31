use crate::exceptions::static_exceptions::{InvalidFunctionChunk, NotFoundChunkForFunction};
use crate::types::JexMachine;
use crate::values::to_output_string::ToOutputString;
use extendable_vm::Exception;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub enum JexValue {
    Null(JexNull),
    Int(i32),
    Bool(bool),
    Object(Rc<JexObject>),
    Instance(Rc<JexInstance>),
    Function(JexFunction),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct JexNull;

#[derive(PartialEq)]
pub enum JexObject {
    String(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JexFunction {
    Script,
    Function {
        arity: usize,
        chunk_id: usize,
        name: String,
    },
}

pub struct JexInstance {
    fields: RefCell<HashMap<String, JexValue>>,
}

impl JexValue {
    pub fn null() -> JexValue {
        JexValue::Null(JexNull)
    }
    pub fn new_object() -> JexValue {
        JexValue::Instance(Default::default())
    }
    pub fn from_string(string: String) -> JexValue {
        JexValue::Object(Rc::new(JexObject::String(string)))
    }
    pub fn as_int(&self) -> Option<i32> {
        if let JexValue::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let JexValue::Bool(bool) = self {
            Some(*bool)
        } else {
            None
        }
    }
    pub fn as_function(&self) -> Option<&JexFunction> {
        if let JexValue::Function(func) = self {
            Some(func)
        } else {
            None
        }
    }
    pub fn as_object(&self) -> Option<&JexObject> {
        if let JexValue::Object(obj) = self {
            Some(&**obj)
        } else {
            None
        }
    }
    pub fn as_string(&self) -> Option<&String> {
        let JexObject::String(string) = self.as_object()?;
        Some(string)
    }
    pub fn as_instance(&self) -> Option<&JexInstance> {
        if let JexValue::Instance(instance) = self {
            Some(&**instance)
        } else {
            None
        }
    }
}

impl JexFunction {
    pub fn from_code(machine: &JexMachine, chunk_id: usize) -> Result<JexFunction, Exception> {
        let chunk = machine
            .code
            .get_chunk(chunk_id)
            .ok_or(NotFoundChunkForFunction(chunk_id))?;
        let name = chunk.constants[0].as_string()?;
        let read_arity = chunk.constants[1].as_int()?;
        let arity = usize::try_from(read_arity);
        if let Ok(usize) = arity {
            Ok(JexFunction::Function {
                chunk_id,
                name,
                arity: usize,
            })
        } else {
            Err(Exception::from(InvalidFunctionChunk(chunk_id)))
        }
    }
}

impl JexInstance {
    pub fn new() -> JexInstance {
        JexInstance {
            fields: RefCell::new(HashMap::new()),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.fields.borrow().is_empty()
    }
    pub fn get_field(&self, name: &str) -> Option<JexValue> {
        self.fields.borrow().get(name).cloned()
    }
    pub fn put_field(&self, name: String, value: JexValue) {
        self.fields.borrow_mut().insert(name, value);
    }
}

impl Default for JexInstance {
    fn default() -> Self {
        JexInstance::new()
    }
}

// TODO: do something with this
impl PartialEq for JexInstance {
    fn eq(&self, _: &Self) -> bool {
        false
    }

    fn ne(&self, _: &Self) -> bool {
        true
    }
}

impl Debug for JexValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JexValue::Int(int) => write!(f, "{}", int),
            JexValue::Bool(bool) => write!(f, "{}", bool),
            JexValue::Null(_) => write!(f, "null"),
            JexValue::Function(func) => write!(f, "{}", func.to_output_string()),
            JexValue::Object(obj) => write!(f, "{:?}", &**obj),
            JexValue::Instance(_) => write!(f, "object"),
        }
    }
}

impl Debug for JexObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JexObject::String(s) => write!(f, "\"{}\"", s),
        }
    }
}
