use std::{env, collections::HashMap};

use neat::tokenizer::serialize;

use crate::neat::datatypes::VType;

pub mod neat;
fn main() {
    let args: Vec<String> = env::args().collect();
    let aliases:HashMap<String, Vec<VType>> = HashMap::new();
    println!("{:?}", serialize(&args[1], &aliases));
}
