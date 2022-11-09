use std::{io::{stdin, self}, env};

use lib::tokenizer::serialize;

pub mod lib;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", serialize(&args[1]).unwrap().to_string());
}
