use crate::code::bytecode_constants::{JexConstant, JexConstantType};
use extendable_vm::parsing_exceptions::CodeEndedAt;
use extendable_vm::{
    ByteReadable, ConstantParser, Exception, ExceptionType, RawBytes, RawBytesPointer,
};

pub type JexConstantParser = ConstantParser<JexConstant>;

pub const JEX_CONSTANT_PARSERS: [JexConstantParser; 3] = [
    ConstantParser {
        constant_type: JexConstantType::Int as u8,
        parser_fn: parse_int_constant,
    },
    ConstantParser {
        constant_type: JexConstantType::String as u8,
        parser_fn: parse_string_constant,
    },
    ConstantParser {
        constant_type: JexConstantType::Function as u8,
        parser_fn: parse_function_constant,
    },
];

fn parse_int_constant(
    bytes: &RawBytes,
    pointer: &mut RawBytesPointer,
) -> Result<JexConstant, Exception> {
    bytes
        .read_i32(pointer)
        .map(JexConstant::Int)
        .ok_or_else(|| Exception::from(CodeEndedAt("i32".to_string())))
}

fn parse_string_constant(
    bytes: &RawBytes,
    pointer: &mut RawBytesPointer,
) -> Result<JexConstant, Exception> {
    let str_len = bytes
        .read_u16(pointer)
        .map(usize::from)
        .ok_or_else(|| CodeEndedAt("u16".to_string()))?;
    let str_data = bytes
        .read_n(pointer, str_len)
        .ok_or_else(|| CodeEndedAt("string data".to_string()))?;
    let str = String::from_utf8(str_data).map_err(|_err| StringConstantParsingError)?;
    Ok(JexConstant::String(str))
}

fn parse_function_constant(
    bytes: &RawBytes,
    pointer: &mut RawBytesPointer,
) -> Result<JexConstant, Exception> {
    let chunk_id = bytes
        .read(pointer)
        .map(usize::from)
        .ok_or_else(|| CodeEndedAt("chunk_id".to_string()))?;
    Ok(JexConstant::Function { chunk_id })
}

pub struct StringConstantParsingError;

impl From<StringConstantParsingError> for Exception {
    fn from(_: StringConstantParsingError) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "StringConstantParsingError".to_string(),
            message: "Could not parse utf8 string".to_string(),
        }
    }
}
