use pyo3::prelude::*;
use std::{collections::HashMap};

use neat::tokenizer::serialize;

use crate::neat::datatypes::VType;

pub mod neat;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn load(_py: Python, file_path:&str) -> PyResult<PyObject> {
    let aliases:HashMap<String, Vec<VType>> = HashMap::new();
    //println!("{:?}", serialize(file_path, &aliases).to_object(_py));
    Ok(serialize(file_path, &aliases).to_object(_py))
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "neat_notation")]
fn neat_notation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    Ok(())
}