use crate::day_10::Instruction::{AddX, NoOp};
use crate::utils::{LineIterator, ParsingLineIterator};
use aoc_runner_derive::aoc;
use std::str::FromStr;

const SIGNAL_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(NoOp),
            _ => Ok(AddX(s.split_at(5).1.parse().unwrap())),
        }
    }
}

struct CommunicationDevice<'input, 'output, O>
where
    O: OutputDevice,
{
    register: i32,
    instructions: ParsingLineIterator<'input, Instruction>,
    output_device: &'output mut O,
}

impl<'input, 'output, O> CommunicationDevice<'input, 'output, O>
where
    O: OutputDevice,
{
    pub fn new(line_iterator: LineIterator<'input>, output_device: &'output mut O) -> Self {
        Self {
            // During first cycle register X is 1
            register: 1,
            instructions: line_iterator.into(),
            output_device,
        }
    }

    pub fn process(mut self) {
        for instruction in self.instructions {
            match instruction {
                NoOp => self.output_device.handle_noop(self.register),
                AddX(value) => {
                    self.output_device.handle_addx(self.register);

                    self.register += value;
                }
            }
        }
    }
}

trait OutputDevice {
    fn handle_noop(&mut self, register: i32);

    fn handle_addx(&mut self, register: i32);
}

#[derive(Debug)]
struct SignalOutputDevice {
    cycles: Vec<i32>,
}

impl SignalOutputDevice {
    pub fn get_signal_strengths(&self) -> impl Iterator<Item = i32> + '_ {
        SIGNAL_CYCLES
            .iter()
            .map(|&cycle| cycle * self.cycles[cycle as usize])
    }
}

impl Default for SignalOutputDevice {
    fn default() -> Self {
        Self { cycles: vec![1] }
    }
}

impl OutputDevice for SignalOutputDevice {
    fn handle_noop(&mut self, register: i32) {
        self.cycles.push(register);
    }

    fn handle_addx(&mut self, register: i32) {
        self.cycles.push(register);
        self.cycles.push(register);
    }
}

#[aoc(day10, part1)]
pub fn solve_part_1(input: &str) -> i32 {
    let mut signal_output_device = SignalOutputDevice::default();

    let communication_device =
        CommunicationDevice::new(LineIterator::from(input), &mut signal_output_device);
    communication_device.process();

    signal_output_device.get_signal_strengths().sum::<i32>()
}

#[derive(Debug)]
struct CrtOutputDevice {
    current_cycle: i32,
    crt: String,
}

impl CrtOutputDevice {
    fn write_to_crt(&mut self, register: i32) {
        // Used to wrap to 40 columns
        let cycle = (self.current_cycle - 1) % 40;

        if [cycle - 1, cycle, cycle + 1].contains(&register) {
            self.crt.push('#');
        } else {
            self.crt.push('.');
        }

        // Break after 40 columns
        if cycle + 1 == 40 {
            self.crt.push('\n');
        }
    }

    fn crt(self) -> String {
        self.crt
    }
}

impl Default for CrtOutputDevice {
    fn default() -> Self {
        // Initialize enough space for the entire screen
        //      240 for the last cycle
        //      240 / 40 for 5 linebreaks + 1 at the start to make the output nice for cargo-aoc
        let mut crt = String::with_capacity(240 + (240 / 40));
        crt.push('\n');

        Self {
            current_cycle: 1,
            crt,
        }
    }
}

impl OutputDevice for CrtOutputDevice {
    fn handle_noop(&mut self, register: i32) {
        self.write_to_crt(register);
        self.current_cycle += 1;
    }

    fn handle_addx(&mut self, register: i32) {
        for _ in 0..2 {
            self.write_to_crt(register);
            self.current_cycle += 1;
        }
    }
}

#[aoc(day10, part2)]
pub fn solve_part_2(input: &str) -> String {
    let mut crt_output_device = CrtOutputDevice::default();

    let communication_device =
        CommunicationDevice::new(LineIterator::from(input), &mut crt_output_device);
    communication_device.process();

    crt_output_device.crt()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
    "#;

    #[test]
    fn test_part_1() {
        let expected = 13140;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
