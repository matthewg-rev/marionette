pub struct Mark {
    pub tag: String,
    pub value: String,
}

impl Mark {
    pub fn new(tag: String, value: String) -> Self {
        Self { tag, value }
    }

    pub fn from_str(tag: &str, value: &str) -> Self {
        Self { tag: tag.to_string(), value: value.to_string() }
    }
}

pub struct MarkedData {
    pub text: String,
    pub marks: Vec<Mark>,
}

impl MarkedData {
    pub fn new(text: String) -> Self {
        Self { text, marks: Vec::new() }
    }

    pub fn resolve_marks(&self) -> String {
        let color_lookup = [
            ("black", 30),
            ("red", 1),
            ("green", 2),
            ("yellow", 3),
            ("blue", 4),
            ("magenta", 5),
            ("cyan", 6),
            ("gray", 7),
        ];

        let mut result = String::new();

        for mark in &self.marks {
            if mark.tag == "color" {
                result.push_str(&format!("\x1b[38;5;{}m", color_lookup.iter().find(|x| x.0 == mark.value).unwrap().1));
            }
        }

        result.push_str(&self.text);

        for mark in &self.marks {
            if mark.tag == "color" {
                result.push_str("\x1b[0m");
            }
        }
        result
    }
}

pub struct Textualizer {
    pub data: Vec<Vec<MarkedData>>,
    pub padding: u64,
}

impl Default for Textualizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Textualizer {
    pub fn new() -> Self {
        Self { data: Vec::new(), padding: 1 }
    }

    pub fn alloc_row(&mut self) {
        self.data.push(Vec::new());
    }

    pub fn add_to_row(&mut self, row: u64, text: String) {
        self.data[row as usize].push(MarkedData::new(text));
    }

    pub fn get_longest_in_column(&mut self, column: u64) -> u64 {
        let mut longest: u64 = 0;
        for row in &self.data {
            if row.len() > column as usize {
                let len = row[column as usize].text.len();
                if len as u64 > longest {
                    longest = len as u64;
                }
            }
        }
        longest
    }
}

impl ToString for Textualizer {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.data {
            for (column_index, column) in row.iter().enumerate() {
                let mut longest: u64 = 0;
                for row_ in &self.data {
                    if row_.len() > column_index {
                        let len = row_[column_index].text.len();
                        if len as u64 > longest {
                            longest = len as u64;
                        }
                    }
                }

                let mut text = column.text.clone();
                while text.len() < longest as usize + self.padding as usize {
                    text.push(' ');
                }

                result.push_str(&text);
                result = result.replace(&column.text, &column.resolve_marks());
            }
            result.push('\n');
        }
        result
    }
}