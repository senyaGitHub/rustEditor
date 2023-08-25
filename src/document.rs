use crate::Row;

#[derive(Default)]

pub struct Document {
	rows: Vec<Row>,
}

impl Document{
	pub fn open() -> Self {
		let mut rows = Vec::new();
		rows.push(Row::from("HeY!!"));
		return Self { rows };
	}
	pub fn row(&self, index: usize) -> Option<&Row>{
		return self.rows.get(index); // if index out of bouce return None
	}
	pub fn is_empty(&self) -> bool {
		self.rows.is_empty()
	}
}