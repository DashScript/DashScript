use std::collections::HashMap;
use std::hash::{ Hash, Hasher };
use crate::common::{ fsize, MAX_BYTES };
use super::vm::VM;

#[derive(Debug, Clone)]
pub struct FloatLike(pub fsize);

impl FloatLike {
    fn to_bytes(&self) -> [u8; MAX_BYTES] {
        self.0.to_le_bytes()
    }
}

// TODO(Scientific-Guy): Make something better for comparison rather than matching bytes.
impl PartialEq for FloatLike {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for FloatLike {}

impl Hash for FloatLike {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.to_bytes().hash(state);
    }
}

pub type NativeFn = fn (this: Value, Vec<Value>, &mut VM) -> Value;

#[derive(Clone, Hash, PartialEq, Debug)]
pub enum ValueIndex {
    Boolean(bool),
    Str(String),
    Num(FloatLike),
    Null
}

impl Eq for ValueIndex {}

#[derive(Clone)]
pub enum Value {
    Boolean(bool),
    Str(String),
    Num(fsize),
    Dict(HashMap<ValueIndex, (u32, bool)>),
    NativeFn(Box<Value>, NativeFn),
    Null
}

impl Value {

    pub fn to_native_fn(func: NativeFn) -> Self {
        Self::NativeFn(Box::new(Value::Null), func)
    }

    pub fn type_as_str(&self) -> String {
        String::from(
            match self {
                Value::Boolean(_) => "boolean",
                Value::Null => "null",
                Value::Str(_) => "string",
                Value::Num(_) => "number",
                Value::NativeFn(_, _) => "function",
                Value::Dict(_) => "object"
            }
        )
    }

    pub fn to_value_index(&self) -> ValueIndex {
        match self {
            Value::Boolean(bool) => ValueIndex::Boolean(*bool),
            Value::Num(num) => ValueIndex::Num(FloatLike(num.clone())),
            Value::Str(str) => ValueIndex::Str(str.clone()),
            _ => ValueIndex::Null
        }
    }

}

#[derive(Clone)]
pub struct ValueRegister {
    pub key: String,
    pub id: u32,
    pub depth: u16,
    pub mutable: bool
}