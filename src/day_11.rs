use crate::utils::LineIterator;
use aoc_runner_derive::aoc;

type Output = usize;

type WorryLevel = usize;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    MultiplyOld,
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<WorryLevel>,
    operation: Operation,
    test_division_value: usize,
    if_test_true_monkey: usize,
    if_test_false_monkey: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_data| {
            let mut iterator = LineIterator::from(monkey_data);
            let _ = iterator.next();

            let starting_items: Vec<WorryLevel> = iterator.next().unwrap()
                ["Starting items: ".len()..]
                .split(", ")
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<WorryLevel>>();
            let operation = {
                let split = iterator.next().unwrap()["Operation: new = old ".len()..]
                    .split_once(' ')
                    .unwrap();
                match split.0 {
                    "*" => match split.1 {
                        "old" => Operation::MultiplyOld,
                        x => Operation::Multiply(x.parse().unwrap()),
                    },
                    _ => Operation::Add(split.1.parse().unwrap()),
                }
            };
            let test_division_value = iterator.next().unwrap()["Test: divisible by ".len()..]
                .parse::<usize>()
                .unwrap();
            let if_test_true_monkey = iterator.next().unwrap()["If true: throw to monkey ".len()..]
                .parse::<usize>()
                .unwrap();
            let if_test_false_monkey = iterator.next().unwrap()
                ["If false: throw to monkey ".len()..]
                .parse::<usize>()
                .unwrap();

            Monkey {
                starting_items,
                operation,
                test_division_value,
                if_test_true_monkey,
                if_test_false_monkey,
            }
        })
        .collect::<Vec<_>>()
}

fn simulate(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    worry_level_division: impl Fn(usize) -> usize,
) -> usize {
    let mut inspections_count: Vec<usize> = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone(); // TODO: Can we get rid of this?

            inspections_count[i] += monkey.starting_items.len();

            for item in &monkey.starting_items {
                let mut worry_level = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::MultiplyOld => item * item,
                };

                worry_level = worry_level_division(worry_level);

                if worry_level % monkey.test_division_value == 0 {
                    monkeys[monkey.if_test_true_monkey]
                        .starting_items
                        .push(worry_level);
                } else {
                    monkeys[monkey.if_test_false_monkey]
                        .starting_items
                        .push(worry_level);
                }
            }

            monkeys[i].starting_items.clear();
        }
    }

    inspections_count.sort_by(|a, b| b.cmp(a));

    inspections_count[0] * inspections_count[1]
}

#[aoc(day11, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let monkeys = parse(input);

    simulate(monkeys, 20, |level| level / 3)
}

#[aoc(day11, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let monkeys = parse(input);

    let modulus = monkeys
        .iter()
        .map(|monkey| monkey.test_division_value)
        .product::<usize>();

    simulate(monkeys, 10_000, |level| level % modulus)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    "#;

    #[test]
    fn test_part_1() {
        let expected = 10605;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 2713310158;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
