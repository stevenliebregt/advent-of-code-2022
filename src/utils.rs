pub enum TrimMode {
    All,
    LineEndOnly,
    None,
}

pub struct LineIteratorSettings {
    pub(crate) trim_mode: TrimMode,
}

impl Default for LineIteratorSettings {
    fn default() -> Self {
        Self {
            trim_mode: TrimMode::All,
        }
    }
}

pub struct LineIterator<'a> {
    input: &'a str,
    settings: LineIteratorSettings,
}

impl<'a> LineIterator<'a> {
    pub fn from(input: &'a str) -> Self {
        Self {
            input,
            settings: LineIteratorSettings::default(),
        }
    }

    pub fn from_settings(input: &'a str, settings: LineIteratorSettings) -> Self {
        Self { input, settings }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let newline_position = self
            .input
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or(self.input.len());

        let (line, rest) = self.input.split_at(newline_position);
        self.input = rest;

        match self.settings.trim_mode {
            TrimMode::All => Some(line.trim()),
            TrimMode::LineEndOnly => Some(line.trim_end_matches(['\r', '\n'])),
            TrimMode::None => Some(line),
        }
    }
}
