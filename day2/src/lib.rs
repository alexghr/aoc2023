use snafu::{prelude::*, Whatever};
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Cube {
    type Err = Whatever;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, color) = s
            .split_once(' ')
            .whatever_context(format!("Failed to read cube count from: {}", s))?;

        let count = count
            .parse::<u32>()
            .whatever_context("Couldn't parse count as u32 from {count}")?;

        match color {
            "red" => Ok(Cube::Red(count)),
            "green" => Ok(Cube::Green(count)),
            "blue" => Ok(Cube::Blue(count)),
            _ => whatever!("Invalid color: {}", color),
        }
    }
}

pub struct Round(Cube, Cube, Cube);

impl FromStr for Round {
    type Err = Whatever;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = Cube::Red(0);
        let mut green = Cube::Green(0);
        let mut blue = Cube::Blue(0);

        for cubes in s.split_terminator(',') {
            let cubes = cubes.trim().parse::<Cube>()?;
            match cubes {
                Cube::Red(_) => red = cubes,
                Cube::Green(_) => green = cubes,
                Cube::Blue(_) => blue = cubes,
            }
        }

        Ok(Round(red, green, blue))
    }
}

pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn is_possible(&self, red: &Cube, green: &Cube, blue: &Cube) -> bool {
        self.rounds.iter().all(|round| {
            let Round(red_count, green_count, blue_count) = round;
            red_count <= red && green_count <= green && blue_count <= blue
        })
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
