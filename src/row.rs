use std::cmp;// compare 

pub struct Row {
	string: String,
}

impl From<&str> for Row {
	fn from(slice: &str) -> Self {
		Self{
			string: String::from(slice),
		}
	}
}

impl Row{
	pub fn render(&self, start: usize, end: usize) -> String{
		let end = cmp::min(end, self.string.len());
		let start = cmp::min(start, end);
		return self.string.get(start..end).unwrap_or_default().to_string();
	}
}