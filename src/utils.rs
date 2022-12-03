use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;

pub fn reader_for_day(day: usize) -> BufReader<File> {
    let file = File::open(filename_for_day(day)).unwrap_or_else(|error| panic!("Failed to open file for day: {day}, looked in: {}, got error: {:#?}", filename_for_day(day), error));
    BufReader::new(file)
}

pub fn filename_for_day(day: usize) -> String {
    let day_as_string = if day < 10 { format!("0{}", day) } else { day.to_string() };
    format!("resources/day_{day_as_string}.txt")
}

// pub struct LineStreamer<'a> {
//     reader: BufReader<File>,
//     buffer: String,
//     phantom_data: PhantomData<&'a str>
// }
//
// impl LineStreamer<'_> {
//     pub fn new_for_day(day: usize) -> Self {
//         Self {
//             reader: reader_for_day(day),
//             buffer: String::new(),
//             phantom_data: PhantomData::default()
//         }
//     }
// }
//
// impl<'a> LineStreamer<'a> {
//     pub fn next(&'a mut self) -> Option<&'a str> {
//         self.buffer.clear();
//         let bytes_read = self.reader.read_line(&mut self.buffer).expect("Could not read line");
//
//         match bytes_read {
//             0 => None,
//             _ => Some(&self.buffer),
//         }
//     }
// }
//
// // impl Iterator for LineStreamer {
// //     type Item<'a> = &'a str;
// //
// //     fn next(&mut self) -> Option<Self::Item> {
// //         self.buffer.clear();
// //         let bytes_read = self.reader.read_line(&mut self.buffer).expect("Could not read line");
// //
// //         match bytes_read {
// //             0 => None,
// //             _ => Some(&self.buffer)
// //         }
// //     }
// // }