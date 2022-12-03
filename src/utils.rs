pub struct LineIterator<'a> {
    input: &'a str
}

impl<'a> LineIterator<'a> {
    pub fn from(input: &'a str) -> Self {
        Self {
            input
        }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let newline_position = self.input.find('\n').map(|i| i + 1).unwrap_or(self.input.len());
        let (line, rest) = self.input.split_at(newline_position);
        self.input = rest;

        Some(line.trim())

        // if self.input.is_empty() { return None }
        //
        // let newline_position = self.input.find('\n').unwrap_or(self.input.len());
        //
        // let (line, rest) = self.input.split_at(newline_position);
        // self.input = rest;
        //
        // Some(line)
    }
}