use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let file_contents = std::fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        for line in file_contents.lines() {
            lines.push(String::from(line));
        }
        Ok(Self { lines })
    }
}
