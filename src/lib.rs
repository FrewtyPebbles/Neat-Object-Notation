pub mod neat;

use std::{collections::HashMap, ffi::{CString, CStr}};

use libc::c_char;
use neat::tokenizer::serialize;
use serde::Serialize;
use serde_json::value::Serializer;

use crate::neat::datatypes::VType;
#[no_mangle]
pub extern fn load(file_path: *const c_char) -> *mut c_char {
    let aliases: HashMap<String, Vec<VType>> = HashMap::new();
	let mut c_string = String::new();
	unsafe {
		c_string = CStr::from_ptr(file_path).to_str().unwrap().to_string();
	}
	println!("{}", c_string);
    CString::new(
        serialize(&c_string, &aliases)
            .serialize(Serializer)
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap()
    .into_raw()
}
