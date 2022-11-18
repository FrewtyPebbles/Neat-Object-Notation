use std::any::{TypeId};


use indexmap::IndexMap;
use pyo3::{types::{IntoPyDict, PyDict, PyList}, IntoPy, ToPyObject, ffi::PyObject};
use pyo3::prelude::*;

pub enum ScopeType {
    None,
    Literal,
    List,
    Struct,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ValWrap {
    None,
    Keyword,
    Literal,
    StringSingle,
    StringDouble,
    Section,
    ListSection,
    EnvVar
}
#[derive(PartialEq, Debug, Clone)]
pub enum PTok {
    SList,
    EList,
    SSection,
    ESection,
    SAlias,
    EAlias,
    ELine,
    SLine,
    Literal,
    Keyword,
    Delimeter,
    Setter,
    AutoInc,
    Blank,
    GlobalList,
    GlobalDict,
    Module(String, Vec<Vec<String>>)
}

#[derive(PartialEq, Debug, Clone)]
pub enum VType {
    Blank,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Alias(String),
    Null,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub v_type: VType,
    pub tok: PTok,
}

pub struct Null {}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum NDSKeyType {
    Int(i64),
    Str(String),
    Bool(bool),
    Null,
    Blank
}

impl From<i64> for NDSKeyType {
    fn from(index: i64) -> Self {
        return NDSKeyType::Int(index);
    }
}
impl From<String> for NDSKeyType {
    fn from(index: String) -> Self {
        return NDSKeyType::Str(index);
    }
}
impl From<Null> for NDSKeyType {
    fn from(_: Null) -> Self {
        return NDSKeyType::Null;
    }
}
impl From<bool> for NDSKeyType {
    fn from(index: bool) -> Self {
        return NDSKeyType::Bool(index);
    }
}

impl Into<i64> for NDSKeyType {
    fn into(self) -> i64 {
        if let NDSKeyType::Int(value) = self {
			return value;
		}
		else {
			return 0;
		}
    }
}
impl Into<String> for NDSKeyType {
    fn into(self) -> String {
        if let NDSKeyType::Str(value) = self{
			return value;
		}
		else {
			return String::new();
		}
    }
}
impl Into<Null> for NDSKeyType {
    fn into(self) -> Null {
        return Null {};
    }
}
impl Into<bool> for NDSKeyType {
    fn into(self) -> bool {
        if let NDSKeyType::Bool(value) = self{
			return value;
		}
		else {
			return false;
		}
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum NDSType {
    // NDSType stands for Node Datastructure Type
    //	 The NDSType is the type of structure that the Node is holding
    //	 and is used by member functions to decern how to access data
    //	 within the node.
    Hashmap(IndexMap<NDSKeyType, Box<SerializedNode>>),
    List(Vec<Box<SerializedNode>>),
    Int(i64),
    Str(String),
    Float(f64),
    Bool(bool),
    Null
}

#[derive(PartialEq, Debug, Clone)]
pub struct SerializedNode {
    // See NDSType comments for info on how this works
    pub value: NDSType,
}



impl SerializedNode {
    //Must use getters and setters to ensure heterogenious datatypes are accessed correctly.
    // EX: Keys in a Hashmap may be of any type
    //Also needs member functions for checking the type of a node.
    fn at<T: Into<usize> + Into<i64> + Into<String> + Into<bool> + Into<Null> + From<i64> + From<String> + From<bool> + From<Null> + 'static>(
        &self,
        index: T,
    ) -> Result<&Box<SerializedNode>, String> {
        match &self.value {
            NDSType::List(val) => {
                return Ok(&val[<T as Into<usize>>::into(index)]);
            }
            NDSType::Hashmap(val) => {
				let generic_t = TypeId::of::<T>();
                if generic_t == TypeId::of::<String>() {
					return Ok(&val[&NDSKeyType::from(<T as Into<String>>::into(index))]);
				}
				else if generic_t == TypeId::of::<bool>() {
					return Ok(&val[&NDSKeyType::from(<T as Into<bool>>::into(index))]);
				}
				else if generic_t == TypeId::of::<i64>() {
					return Ok(&val[&NDSKeyType::from(<T as Into<i64>>::into(index))]);
				}
				else if generic_t == TypeId::of::<Null>() {
					return Ok(&val[&NDSKeyType::from(<T as Into<Null>>::into(index))]);
				}
				else {
					return Err(String::from("i128, String, bool, datatypes::Null are the only compatable types with Hashmap Nodes"));
				}
            }
            _ => {
                return Err(String::from("Not an indexable type"));
            }
        };
    }
}


impl ToPyObject for SerializedNode {
    fn to_object(&self, py: Python<'_>) -> pyo3::PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for NDSType {
    fn to_object(&self, py: Python<'_>) -> pyo3::PyObject {
        match self {
            NDSType::Hashmap(map) => {
                let mut python_dict:Vec<(Py<PyAny>, Py<PyAny>)> = vec![];
                for (key, val) in map.iter() {
                    python_dict.push((key.to_object(py), val.to_object(py)));
                }
                return python_dict.into_py_dict(py).to_object(py);
            },
            NDSType::List(list) => {
                let mut ret_list:Vec<Py<PyAny>> = vec![];
                for item in list.iter() {
                    ret_list.push(item.to_object(py));
                }
                return ret_list.to_object(py);
            },
            NDSType::Int(val) => {
                let py_obj = val.into_py(py);
                return py_obj;
            },
            NDSType::Str(val) => {return val.into_py(py)},
            NDSType::Float(val) => {return val.into_py(py)},
            NDSType::Bool(val) => {return val.into_py(py)},
            NDSType::Null => {return "None".into_py(py)},
        }
    }
}

impl ToPyObject for NDSKeyType {
    fn to_object(&self, py: Python<'_>) -> pyo3::PyObject {
        match self {
            NDSKeyType::Int(val) => {return val.into_py(py)},
            NDSKeyType::Str(val) => {return val.into_py(py)},
            NDSKeyType::Bool(val) => {return val.into_py(py)},
            _ => {return 0.into_py(py)}
        }
    }
}