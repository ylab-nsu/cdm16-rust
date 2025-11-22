pub trait Indent {
    fn indent_lines(&self, count: u8) -> String;
}

impl Indent for str {
    fn indent_lines(&self, count: u8) -> String {
        let mut result = self.lines().fold(String::new(), |mut string, line| {
            for _ in 0..count {
                string.push(' ');
            }
            string.push_str(line);
            string.push('\n');
            string
        });
        result.pop();
        result
    }
}

