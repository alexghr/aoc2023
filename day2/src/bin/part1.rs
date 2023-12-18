use day2::Game;

fn main() {
    let input = include_str!("../../input/part1.txt");
    let result = process(input);
    println!("Result (part1): {}", result);
}

fn process(input: &str) -> String {
    let mut sum = 0;
    for game in input.lines() {
        let game = game.parse::<Game>().unwrap();
        if game.is_possible(12, 13, 14) {
            sum += game.id;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "8"
        );
    }
}
