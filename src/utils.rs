use std::fmt::Debug;
use std::marker::PhantomData;
use std::str::FromStr;

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

    #[inline] // TODO: Does this matter?
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

pub struct ParsingLineIterator<'a, T> {
    line_iterator: LineIterator<'a>,
    marker: PhantomData<T>,
}

impl<'a, T> ParsingLineIterator<'a, T> {
    pub fn from(input: &'a str) -> Self {
        Self {
            line_iterator: LineIterator::from(input),
            marker: Default::default(),
        }
    }

    pub fn from_settings(input: &'a str, settings: LineIteratorSettings) -> Self {
        Self {
            line_iterator: LineIterator::from_settings(input, settings),
            marker: Default::default(),
        }
    }
}

impl<'a, T> From<LineIterator<'a>> for ParsingLineIterator<'a, T> {
    fn from(line_iterator: LineIterator<'a>) -> Self {
        Self {
            line_iterator,
            marker: PhantomData::<T>::default(),
        }
    }
}

impl<'a, T> Iterator for ParsingLineIterator<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.line_iterator.next() {
            return Some(line.parse::<T>().unwrap());
        }

        None
    }
}
