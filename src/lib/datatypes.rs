use std::any::{TypeId};

use indexmap::IndexMap;

use serde::{Serialize, Serializer, ser::{SerializeMap, SerializeSeq}};

pub enum ScopeType {
    None,
    Literal,
    List,
    Struct,
}

#[derive(PartialEq)]
pub enum ValWrap {
    None,
    Keyword,
    Literal,
    StringSingle,
    StringDouble,
    Section,
    ListSection,
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

//SERIALIZATION
impl Serialize for SerializedNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let typename = std::any::type_name::<S>();
        if typename.contains("serde_json") {
            match self.value.clone() {
                NDSType::Hashmap(val) => {
                    let mut map = serializer.serialize_map(Some(val.len()))?;
                    for (key, value) in val.iter() {
                        match key {
                            NDSKeyType::Int(kval) => {map.serialize_entry(kval, value)?;},
                            NDSKeyType::Str(kval) => {map.serialize_entry(kval, value)?;},
                            NDSKeyType::Bool(kval) => {},
                            NDSKeyType::Null => {map.serialize_entry(&Option::<char>::None, value)?;},
                            NDSKeyType::Blank => {},
                        };
                    }
                    map.end()
                },
                NDSType::List(val) => {
                    let mut seq = serializer.serialize_seq(Some(val.len()))?;
                    for value in val.iter() {
                        seq.serialize_element(value)?;
                    }
                    seq.end()
                },
                NDSType::Int(val) => {
                    serializer.serialize_i64(val)
                },
                NDSType::Str(val) => {
                    serializer.serialize_str(val.as_str())
                },
                NDSType::Float(val) => {
                    serializer.serialize_f64(val)
                },
                NDSType::Bool(val) => {
                    serializer.serialize_bool(val)
                },
                NDSType::Null => {
                    serializer.serialize_none()
                },
            }
        }
        else {
            match self.value.clone() {
                NDSType::Hashmap(mut val) => {
                    let mut map = serializer.serialize_map(Some(val.len()))?;
                    for (key, value) in val.iter() {
                        match key {
                            NDSKeyType::Int(kval) => {map.serialize_entry(kval, value)?;},
                            NDSKeyType::Str(kval) => {map.serialize_entry(kval, value)?;},
                            NDSKeyType::Bool(kval) => {map.serialize_entry(kval, value)?;},
                            NDSKeyType::Null => {map.serialize_entry(&Option::<char>::None, value)?;},
                            NDSKeyType::Blank => {map.serialize_entry(&Option::<char>::None, value)?;},
                        };
                    }
                    map.end()
                },
                NDSType::List(val) => {
                    let mut seq = serializer.serialize_seq(Some(val.len()))?;
                    for value in val.iter() {
                        seq.serialize_element(value)?;
                    }
                    seq.end()
                },
                NDSType::Int(val) => {
                    serializer.serialize_i64(val)
                },
                NDSType::Str(val) => {
                    serializer.serialize_str(val.as_str())
                },
                NDSType::Float(val) => {
                    serializer.serialize_f64(val)
                },
                NDSType::Bool(val) => {
                    serializer.serialize_bool(val)
                },
                NDSType::Null => {
                    serializer.serialize_none()
                },
            }
        }
        
    }
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
