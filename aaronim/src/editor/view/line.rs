use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;
pub struct Line {
    string: String,
}

impl Line {
    pub fn from(line: &str) -> Self {
        Self {
            string: String::from(line),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let graphemes = self.string.graphemes(true).collect::<Vec<&str>>();
        let start = range.start;
        let end = cmp::min(range.end, graphemes.len());
        graphemes[start..end].concat()
    }

    pub fn len(&self) -> usize {
        self.string.graphemes(true).count()
    }
}
