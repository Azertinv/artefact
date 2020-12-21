use crate::byte::Byte;
use crate::word::Word;
use crate::register::Registers;

use json::{JsonError, JsonValue};

use gdnative::prelude::*;

pub struct DataChunk {
    pub memspace: usize,
    pub addr: isize,
    pub data: Vec<Byte>,
}

pub struct PermChunk {
    pub memspace: usize,
    pub addr: isize,
    pub permissions: Int32Array,
}

pub struct Program {
    pub regs: Registers,
    pub data_chunks: Vec<DataChunk>,
    pub perm_chunks: Vec<PermChunk>,
}

impl Program {
    pub fn load_from_json(json_str: String) -> json::Result<Program> {
        let json_object = json::parse(&json_str)?;

        // load registers from the compiled file
        let mut regs = Registers::new();
        regs.pc = Word::from(json_object["regs"]["pc"]
            .as_i64().ok_or(JsonError::WrongType("pc".to_string()))?);
        regs.sp = Word::from(json_object["regs"]["sp"]
            .as_i64().ok_or(JsonError::WrongType("sp".to_string()))?);
        regs.flags = Word::from(json_object["regs"]["flags"]
            .as_i64().ok_or(JsonError::WrongType("flags".to_string()))?);
        regs.a = Word::from(json_object["regs"]["a"]
            .as_i64().ok_or(JsonError::WrongType("a".to_string()))?);
        regs.b = Word::from(json_object["regs"]["b"]
            .as_i64().ok_or(JsonError::WrongType("b".to_string()))?);
        regs.c = Word::from(json_object["regs"]["c"]
            .as_i64().ok_or(JsonError::WrongType("c".to_string()))?);
        regs.d = Word::from(json_object["regs"]["d"]
            .as_i64().ok_or(JsonError::WrongType("d".to_string()))?);
        regs.e = Word::from(json_object["regs"]["e"]
            .as_i64().ok_or(JsonError::WrongType("e".to_string()))?);
        regs.f = Word::from(json_object["regs"]["f"]
            .as_i64().ok_or(JsonError::WrongType("f".to_string()))?);

        // load data chunks from the compiled file
        let mut data_chunks: Vec<DataChunk> = vec!();
        if let JsonValue::Array(json_data_chunks) = &json_object["data_chunks"] {
            for json_data_chunk in json_data_chunks {
                let memspace: usize = json_data_chunk["memspace"]
                    .as_usize().ok_or(JsonError::WrongType("memspace".to_string()))?;
                let addr: isize = json_data_chunk["addr"]
                    .as_isize().ok_or(JsonError::WrongType("addr".to_string()))?;
                if !json_data_chunk["data"].is_array() {
                    return Err(JsonError::WrongType("data".to_string()));
                }
                let mut data: Vec<Byte> = vec!();
                for i in 0..json_data_chunk["data"].len() {
                    data.push(Byte::from(json_data_chunk["data"][i]
                            .as_i64().ok_or(JsonError::WrongType("data".to_string()))?));
                }
                data_chunks.push(DataChunk{memspace, addr, data});
            }
        };

        // load permissions from the compiled file
        let mut perm_chunks: Vec<PermChunk> = vec!();
        if let JsonValue::Array(json_perm_chunks) = &json_object["perm_chunks"] {
            for json_perm_chunk in json_perm_chunks {
                let memspace: usize = json_perm_chunk["memspace"]
                    .as_usize().ok_or(JsonError::WrongType("memspace".to_string()))?;
                let addr: isize = json_perm_chunk["addr"]
                    .as_isize().ok_or(JsonError::WrongType("addr".to_string()))?;
                if !json_perm_chunk["perm"].is_array() {
                    return Err(JsonError::WrongType("perm".to_string()));
                }
                let mut permissions: Int32Array = Int32Array::new();
                for i in 0..json_perm_chunk["perm"].len() {
                    permissions.push(json_perm_chunk["perm"][i]
                            .as_i32().ok_or(JsonError::WrongType("perm".to_string()))?);
                }
                perm_chunks.push(PermChunk{memspace, addr, permissions});
            }
        };
        Ok(Program {regs, data_chunks, perm_chunks})
    }
}
