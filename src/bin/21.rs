use std::collections::{BTreeMap, VecDeque};

use nom::{
    bytes::complete::{tag, take, take_till1},
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Monkey> {
    let (input, (name, _, entry)) = tuple((alpha1, tag(": "), take_till1(|c| c == '\n')))(input)?;
    if let Ok(value) = entry.parse::<i64>() {
        Ok((
            input,
            Monkey {
                name,
                calculated_value: Some(value),
                ..Default::default()
            },
        ))
    } else {
        let (_, (lhs, op, rhs)) = tuple((alpha1, take(3usize), alpha1))(entry)?;
        Ok((
            input,
            Monkey {
                name,
                left: Some(lhs),
                right: Some(rhs),
                operation: Some(op.into()),
                ..Default::default()
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOperationError;

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value.trim() {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Invalid Operation"),
        }
    }
}

impl Operation {
    fn run(&self, left: i64, right: i64) -> i64 {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Monkey<'a> {
    name: &'a str,
    left: Option<&'a str>,
    right: Option<&'a str>,
    operation: Option<Operation>,
    calculated_value: Option<i64>,
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, monkies) = separated_list1(line_ending, parse_line)(input).unwrap();

    // dbg!(&monkies);

    let mut monkies_by_name = BTreeMap::new();
    let mut value_map = BTreeMap::new();
    for monkey in &monkies {
        monkies_by_name.insert(monkey.name, monkey);
        if let Some(value) = monkey.calculated_value {
            value_map.insert(monkey.name, value);
        }
    }

    let mut uncalculated_monkies = VecDeque::from(["root"]);

    while let Some(monkey_name) = uncalculated_monkies.pop_front() {
        let monkey = monkies_by_name.get(monkey_name).unwrap();

        let left_name = monkey.left.unwrap();
        let left_value = value_map.get(left_name);
        let right_name = monkey.right.unwrap();
        let right_value = value_map.get(right_name);

        match (left_value, right_value) {
            (Some(&left), Some(&right)) => {
                value_map.insert(monkey_name, monkey.operation.unwrap().run(left, right));
            }
            (None, None) => {
                uncalculated_monkies.push_front(monkey_name);
                uncalculated_monkies.push_front(left_name);
                uncalculated_monkies.push_front(right_name);
            }
            (None, _) => {
                uncalculated_monkies.push_front(monkey_name);
                uncalculated_monkies.push_front(left_name);
            }
            (_, None) => {
                uncalculated_monkies.push_front(monkey_name);
                uncalculated_monkies.push_front(right_name);
            }
        }
    }

    // dbg!(&value_map);
    Some(*(value_map.get("root").unwrap()))
}

pub fn part_two(_input: &str) -> Option<i64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    #[ignore]
    fn test_solutions() {
        let input = advent_of_code::read_file("inputs", 21);
        assert_eq!(part_one(&input), Some(10037517593724));
        assert_eq!(part_two(&input), None);
    }
}
