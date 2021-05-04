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
    // TODO(Scientific-Guy): Think a better way for native functions.
    NativeFn(Box<Value>, NativeFn),
    // Array is used as a value type instead of an object because to prevent unwanted memory of attributes in value register.
    // TODO(Scientific-Guy): Find a way to make array as an object instead of a value type.
    Array(Vec<u32>),
    Func(u32, Vec<(u32, bool)>, Vec<u8>, bool),
    Null
}

impl From<bool> for Value { 
    fn from(bool: bool) -> Self { Self::Boolean(bool) } 
}

impl From<fsize> for Value { 
    fn from(num: fsize) -> Self { Self::Num(num) } 
}

impl From<String> for Value {
    fn from(str: String) -> Self { Self::Str(str) }
}

impl From<NativeFn> for Value {
    fn from(func: NativeFn) -> Self { Self::NativeFn(Box::new(Value::Null), func) }
}

impl Default for Value {
    fn default() -> Self { Value::Null }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Func(_, _, a, _), Value::Func(_, _, b, _)) => a == b,
            (Value::NativeFn(_, a), Value::NativeFn(_, b)) => a as *const NativeFn == b as *const NativeFn,
            (Value::Dict(a), Value::Dict(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false
        }
    }
}

impl Eq for Value {}

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
                Value::NativeFn(_, _) | Value::Func(_, _, _, _) => "function",
                Value::Dict(_) => "object",
                Value::Array(_) => "array"
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

    pub fn to_vec(&self, vm: &mut VM) -> Vec<Self> {
        match self {
            Value::Array(arr) => {
                let mut res = vec![];
                for item in arr {
                    res.push(vm.value_stack.get(*item as usize).unwrap_or(&Value::Null).clone())
                }

                res
            },
            _ => vec![]
        }
    }

}

#[derive(Clone)]
pub struct ValueRegister {
    pub key: String,
    pub id: u32,
    pub mutable: bool,
    pub depth: u32
}

pub enum Break {
    Break,
    Return(Value),
    None
}

impl Break {
    pub fn is_some(&self) -> bool {
        match self {
            Break::None => true,
            _ => false
        }
    }
}