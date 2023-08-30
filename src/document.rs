use crate::Row;
use std::fs;

#[derive(Default)]

pub struct Document {
	rows: Vec<Row>,
	pub file_name: Option<String>,
}

impl Document{
	pub fn open(filename: &str) -> Result <Self, std::io::Error>{
		let contents = fs::read_to_string(filename)?;
		let mut rows = Vec::new();
		for value in contents.lines(){
			rows.push(Row::from(value));
		}
		return Ok(Self{rows, file_name: Some(filename.to_string())});
	}
	pub fn row(&self, index: usize) -> Option<&Row>{
		return self.rows.get(index); // if index out of bouce return None
	}
	pub fn is_empty(&self) -> bool {
		return self.rows.is_empty();
	}
	pub fn len(&self) -> usize{
		return self.rows.len();
	}
}