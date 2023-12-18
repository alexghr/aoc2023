use std::str::FromStr;

use snafu::Whatever;

pub struct Schematic {
    map: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    line: usize,
    column: usize,
    width: usize,
    pub value: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Symbol {
    line: usize,
    column: usize,
}

impl Schematic {
    pub fn part_numbers(&self) -> Vec<Number> {
        let mut i = 0;
        let mut result = Vec::new();

        while i < self.map.len() {
            let mut j = 0;
            while j < self.map[i].len() {
                if let Some(num) = self.find_next_number(i, j) {
                    j = num.column + num.width;

                    if self.is_part_number(&num) {
                        result.push(num);
                    }
                } else {
                    break;
                }
            }

            i += 1;
        }

        result
    }

    pub fn gear_ratios(&self) -> u32 {
        let part_numbers = self.part_numbers();
        let mut sum = 0;
        for i in 0..part_numbers.len() {
            for j in i + 1..part_numbers.len() {
                if i == j {
                    continue;
                }

                if self.gear_connected(&part_numbers[i], &part_numbers[j]) {
                    sum += part_numbers[i].value * part_numbers[j].value;
                }
            }
        }

        sum
    }

    fn find_next_number(&self, line: usize, column: usize) -> Option<Number> {
        let mut start = 0;
        let mut width = 0;
        for i in column..self.map[line].len() {
            if self.map[line][i].is_numeric() && width > 0 {
                width += 1;
            } else if self.map[line][i].is_numeric() {
                start = i;
                width = 1;
            } else if width > 0 {
                break;
            }
        }

        if width > 0 {
            let start = start as usize;
            Some(Number {
                line,
                column: start,
                width,
                value: self.get_number(line, start, width),
            })
        } else {
            None
        }
    }

    fn gear_connected(&self, a: &Number, b: &Number) -> bool {
        let a_symbols = self.get_symbols_near(a);
        let b_symbols = self.get_symbols_near(b);

        a_symbols.iter().any(|a_symbol| {
            self.is_gear(a_symbol) && b_symbols.iter().any(|b_symbol| a_symbol == b_symbol)
        })
    }

    fn get_number(&self, line: usize, column: usize, width: usize) -> u32 {
        let number: u32 = self.map[line][column..column + width]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        number
    }

    fn is_gear(&self, symbol: &Symbol) -> bool {
        self.map[symbol.line][symbol.column] == '*'
    }

    fn get_symbols_near(&self, number: &Number) -> Vec<Symbol> {
        let Number {
            column,
            line,
            width,
            ..
        } = number;

        let line = *line as i32;
        let column = *column as i32;

        vec![
            self.find_symbols(line - 1, column - 1, width + 2),
            self.find_symbols(line + 1, column - 1, width + 2),
            self.find_symbols(line, column - 1, 1),
            self.find_symbols(line, column + *width as i32, 1),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn is_part_number(&self, number: &Number) -> bool {
        self.get_symbols_near(number).len() > 0
    }

    fn find_symbols(&self, line: i32, column: i32, width: usize) -> Vec<Symbol> {
        let mut results = Vec::new();

        if line < 0 || line >= self.map.len() as i32 {
            return results;
        }

        let line = line as usize;
        for j in column..column + width as i32 {
            if j < 0 || j >= self.map[line].len() as i32 {
                continue;
            }

            let j = j as usize;
            if !self.map[line][j].is_numeric() && self.map[line][j] != '.' {
                results.push(Symbol { line, column: j });
            }
        }

        results
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number() {
        let schematic: Schematic = "467..114..".parse().unwrap();
        assert_eq!(
            schematic.find_next_number(0, 0),
            Some(Number {
                line: 0,
                column: 0,
                width: 3,
                value: 467,
            })
        );

        assert_eq!(
            schematic.find_next_number(0, 3),
            Some(Number {
                line: 0,
                column: 5,
                width: 3,
                value: 114,
            })
        );
    }
}
