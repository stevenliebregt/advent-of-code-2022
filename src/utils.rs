use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Sub};
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

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    pub fn set_x(&mut self, new_x: isize) {
        self.x = new_x
    }

    pub fn set_y(&mut self, new_y: isize) {
        self.y = new_y
    }
}

impl Sub<(isize, isize)> for Coordinate {
    type Output = Self;

    fn sub(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl Add<(isize, isize)> for Coordinate {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(',').unwrap();

        Ok(Self {
            x: split.0.parse().unwrap(),
            y: split.1.parse().unwrap(),
        })
    }
}

impl From<(isize, isize)> for Coordinate {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

pub fn manhattan_distance(source: &Coordinate, target: &Coordinate) -> isize {
    let x = (source.x() - target.x()).abs();
    let y = (source.y() - target.y()).abs();

    x + y
}

pub fn diamond_x_bounds(
    coordinate: Coordinate,
    radius: usize,
    vertical_distance_from_center: usize,
) -> (isize, isize) {
    let modifier = radius as isize - vertical_distance_from_center as isize;

    (coordinate.x() - modifier, coordinate.x() + modifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Imagine a diamond that looks like this, with radius 5 at (0, 0)
    ///
    /// ```txt
    ///      #
    ///     ###
    ///    #####
    ///   #######
    ///  #########
    /// #####X#####
    ///  #########
    ///   #######
    ///    #####
    ///     ###
    ///      #
    /// ```
    #[test]
    fn test_diamond_bounds() {
        let coordinate = Coordinate::new(0, 0);
        let radius = 5;

        // We then should get that 3 up the x coordinates would be -2 and 2
        assert_eq!((-2, 2), diamond_x_bounds(coordinate, radius, 3));
    }
}