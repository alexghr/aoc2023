use snafu::{prelude::*, Whatever};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = Whatever;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => whatever!("Invalid color: {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cubes {
    color: Color,
    count: u32,
}

impl FromStr for Cubes {
    type Err = Whatever;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, color) = s
            .split_once(' ')
            .whatever_context(format!("Failed to read cube count from: {}", s))?;

        let count = count
            .parse::<u32>()
            .whatever_context("Couldn't parse count as u32 from {count}")?;

        let color = color.parse::<Color>()?;

        Ok(Cubes { color, count })
    }
}

struct Round {
    cubes: Vec<Cubes>,
}

impl Round {
    fn count(&self, color: Color) -> u32 {
        self.cubes
            .iter()
            .filter(|cubes| cubes.color == color)
            .fold(0, |sum, cubes| sum + cubes.count)
    }
}

impl FromStr for Round {
    type Err = Whatever;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .split_terminator(',')
            .map(|cube| cube.trim().parse::<Cubes>())
            .collect::<Result<Vec<Cubes>, Whatever>>()?;

        Ok(Round { cubes })
    }
}

pub struct Game {
    pub id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.rounds.iter().all(|round| {
            round.count(Color::Red) < red
                && round.count(Color::Green) < green
                && round.count(Color::Blue) < blue
        })
    }

    pub fn power(&self) -> u32 {
        let red = self
            .rounds
            .iter()
            .map(|round| round.count(Color::Red))
            .max()
            .map_or(1, |x| if x == 0 { 1 } else { x });

        let green = self
            .rounds
            .iter()
            .map(|round| round.count(Color::Green))
            .max()
            .map_or(1, |x| if x == 0 { 1 } else { x });

        let blue = self
            .rounds
            .iter()
            .map(|round| round.count(Color::Blue))
            .max()
            .map_or(1, |x| if x == 0 { 1 } else { x });

        red * green * blue
    }
}

impl FromStr for Game {
    type Err = Whatever;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s
            .split_once(':')
            .whatever_context(format!("Failed to read game id from: {}", s))?;

        let id = id
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .whatever_context(format!("Failed to read game id: {})", id))?;

        let rounds = rounds
            .split(';')
            .map(|round| round.trim().parse::<Round>())
            .collect::<Result<Vec<Round>, Whatever>>()?;

        Ok(Game { id, rounds })
    }
}
