use std::collections::HashSet;

use nom::{
    character::complete::{alpha1, digit1, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Copy, Clone, Debug)]
enum Move {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

impl Move {
    fn amount(self: Move) -> u16 {
        match self {
            Move::Up(amount) => amount,
            Move::Down(amount) => amount,
            Move::Left(amount) => amount,
            Move::Right(amount) => amount,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Vector2D {
    x: i16,
    y: i16,
}

impl Vector2D {
    fn is_adjacent(self: Vector2D, other: Vector2D) -> bool {
        (self.x - 1..=self.x + 1).contains(&other.x) && (self.y - 1..=self.y + 1).contains(&other.y)
    }

    fn move_one(self: &mut Vector2D, m: Move) {
        match m {
            Move::Up(_) => self.y += 1,
            Move::Down(_) => self.y -= 1,
            Move::Left(_) => self.x -= 1,
            Move::Right(_) => self.x += 1,
        }
    }
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        separated_pair(alpha1, space1, digit1),
        |(m, amount): (&str, &str)| {
            let amount = amount.parse::<u16>().unwrap();
            match m {
                "U" => Move::Up(amount),
                "D" => Move::Down(amount),
                "L" => Move::Left(amount),
                "R" => Move::Right(amount),
                _ => panic!("Unknown move type {}", m),
            }
        },
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, moves) = separated_list1(newline, parse_move)(input).unwrap();

    let mut visited = HashSet::<Vector2D>::new();
    let mut head = Vector2D { x: 0, y: 0 };
    let mut tail = Vector2D { x: 0, y: 0 };

    visited.insert(tail);

    for m in moves {
        for _ in 0..m.amount() {
            let prev_head = head.clone();
            head.move_one(m);

            if !tail.is_adjacent(head) {
                tail = prev_head;
                visited.insert(tail);
            }
        }
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    #[ignore]
    fn test_solutions() {
        let input = advent_of_code::read_file("inputs", 9);
        assert_eq!(part_one(&input), Some(6367));
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_is_adjacent() {
        let p1 = Vector2D { x: 0, y: 0 };

        for x in -1..=1 {
            for y in -1..=1 {
                let p2 = Vector2D { x, y };
                assert_eq!(p1.is_adjacent(p2), true);
            }

            assert_eq!(p1.is_adjacent(Vector2D { x, y: -2 }), false);
            assert_eq!(p1.is_adjacent(Vector2D { x, y: 2 }), false);
        }

        for y in -1..=1 {
            assert_eq!(p1.is_adjacent(Vector2D { x: -2, y }), false);
            assert_eq!(p1.is_adjacent(Vector2D { x: 2, y }), false);
        }
    }
}
