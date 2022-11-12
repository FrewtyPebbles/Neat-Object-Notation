use std::{io::{stdin, self}, env, collections::HashMap};

use lib::tokenizer::serialize;
use serde::Serialize;
use serde_json::value::Serializer;

use crate::lib::datatypes::VType;

pub mod lib;
fn main() {
    let args: Vec<String> = env::args().collect();
    let aliases:HashMap<String, Vec<VType>> = HashMap::new();
    println!("{}", serialize(&args[1], &aliases).serialize(Serializer).unwrap().to_string());
}
