use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        { // tab == 4 spaces
            if grapheme == "\t"{
            	result.push_str("    ")
            } else{
            	result.push_str(grapheme);
            }
        }
        result
    }
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }
    fn update_len(&mut self) {
        return self.len = self.string[..].graphemes(true).count();
    }
}