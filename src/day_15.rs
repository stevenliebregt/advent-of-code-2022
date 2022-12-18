use crate::utils::{manhattan_distance, Coordinate};
use advent_of_code_helpers::parsing_line_iterator::ParsingLineIterator;
use aoc_runner_derive::aoc;
use std::str::FromStr;

type Output = isize;

#[derive(Debug)]
struct Sensor {
    at: Coordinate,
    closest_beacon: Coordinate,
    manhattan_distance: isize,
}

impl Sensor {
    fn distance_to(&self, other: &Coordinate) -> isize {
        manhattan_distance(&self.at, other)
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_str, beacon_str) = s.split_once(": ").unwrap();

        let at: Coordinate = sensor_str["Sensor at ".len()..]
            .replace("x=", "")
            .replace(" y=", "")
            .parse()
            .unwrap();
        let closest_beacon: Coordinate = beacon_str["closest beacon is at ".len()..]
            .replace("x=", "")
            .replace(" y=", "")
            .parse()
            .unwrap();

        Ok(Self {
            manhattan_distance: manhattan_distance(&at, &closest_beacon),
            at,
            closest_beacon,
        })
    }
}

fn in_range_of_sensor(sensor: &Sensor, coordinate: &Coordinate) -> bool {
    sensor.distance_to(coordinate) <= sensor.manhattan_distance && &sensor.closest_beacon != coordinate
}

#[aoc(day15, part1)]
pub fn solve_part_1_aoc(input: &str) -> Output {
    solve_part_1(input, 2_000_000)
}

pub fn solve_part_1_test(input: &str) -> Output {
    solve_part_1(input, 10)
}

pub fn solve_part_1(input: &str, row: isize) -> Output {
    let sensors = ParsingLineIterator::<Sensor>::from(input).collect::<Vec<_>>();

    let minimum_x = sensors
        .iter()
        .map(|sensor| sensor.at.x() - sensor.manhattan_distance)
        .min()
        .unwrap();
    let maximum_x = sensors
        .iter()
        .map(|sensor| sensor.at.x() + sensor.manhattan_distance)
        .max()
        .unwrap();

    (minimum_x..=maximum_x)
        .map(|x| Coordinate::new(x, row))
        .filter(|coordinate| {
            sensors
                .iter()
                .any(|sensor| in_range_of_sensor(sensor, coordinate))
        })
        .count() as isize
}

fn is_valid(sensors: &[Sensor], coordinate: &Coordinate) -> bool {
    sensors.iter().all(|sensor| {
        let distance = sensor.distance_to(coordinate);

        distance > sensor.manhattan_distance
    })
}

#[aoc(day15, part2)]
pub fn solve_part_2_aoc(input: &str) -> Output {
    solve_part_2(input, (0, 4_000_000))
}

pub fn solve_part_2_test(input: &str) -> Output {
    solve_part_2(input, (0, 20))
}

pub fn solve_part_2(input: &str, min_max_y: (isize, isize)) -> Output {
    let sensors = ParsingLineIterator::<Sensor>::from(input).collect::<Vec<_>>();

    let mut valid_coordinate = Coordinate::default();

    for Sensor { at, manhattan_distance, ..} in &sensors {
        let mut y = min_max_y.0;

        for x in at.x() - manhattan_distance - 1..at.x().min(min_max_y.1) {
            if x < 0 {
                y += 1;
                continue;
            }

            let coordinate = Coordinate::new(x, at.y() + y);
            if coordinate.y() <= min_max_y.1 && is_valid(&sensors, &coordinate) {
                valid_coordinate = coordinate;
                break;
            }

            let coordinate = Coordinate::new(x, at.y() - y);
            if coordinate.y() >= min_max_y.0 && is_valid(&sensors, &coordinate) {
                valid_coordinate = coordinate;
                break;
            }

            y += 1;
        }
    }

    valid_coordinate.x() * 4_000_000 + valid_coordinate.y()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "#;

    #[test]
    fn test_part_1() {
        let expected = 26;

        assert_eq!(expected, solve_part_1_test(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 56000011;

        assert_eq!(expected, solve_part_2_test(INPUT.trim()));
    }
}
