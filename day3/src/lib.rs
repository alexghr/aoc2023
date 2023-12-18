use std::str::FromStr;

use snafu::Whatever;

pub struct Schematic {
    map: Vec<Vec<char>>,
}

impl Schematic {
    pub fn sum(&self) -> u32 {
        let mut sum = 0;
        let mut i = 0;

        while i < self.map.len() {
            let mut j = 0;
            while j < self.map[i].len() {
                if self.map[i][j].is_numeric() {
                    let width = self.find_width(i, j);
                    let part_number: u32 = self.map[i][j..j + width]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    if self.is_part_number(i as i32, j as i32, width) {
                        sum += part_number;
                    }
                    j += width;
                } else {
                    j += 1;
                }
            }

            i += 1;
        }

        sum
    }

    fn find_width(&self, line: usize, col: usize) -> usize {
        let mut width = 0;
        for i in col..self.map[line].len() {
            if self.map[line][i].is_numeric() {
                width += 1;
            } else {
                break;
            }
        }

        width
    }

    fn is_part_number(&self, line: i32, column: i32, width: usize) -> bool {
        vec![
            self.find_symbol(line - 1, column - 1, width + 2),
            self.find_symbol(line + 1, column - 1, width + 2),
            self.find_symbol(line, column - 1, 1),
            self.find_symbol(line, column + width as i32, 1),
        ]
        .iter()
        .any(|x| x.is_some())
    }

    fn find_symbol(&self, line: i32, column: i32, width: usize) -> Option<char> {
        if line < 0 || line >= self.map.len() as i32 {
            return None;
        }

        let line = line as usize;
        for j in column..column + width as i32 {
            if j < 0 || j >= self.map[line].len() as i32 {
                continue;
            }

            let j = j as usize;
            if !self.map[line][j].is_numeric() && self.map[line][j] != '.' {
                return Some(self.map[line][j]);
            }
        }

        None
    }
}

impl FromStr for Schematic {
    type Err = Whatever;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Schematic { map })
    }
}
