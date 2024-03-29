use anyhow::Result;
use std::fs;

pub enum Element {
    Vertice(usize, usize, usize),
    Face(usize, usize, usize),
}

pub fn parse_file(file_name: &str) -> Result<String> {
    let contents = fs::read_to_string(file_name)?;
    return Ok(contents);
}
