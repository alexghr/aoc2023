use regex::Regex;

fn main() {
    let input = include_str!("../../input/part2.txt");
    let result = process(input);
    println!("Result (part2): {}", result);
}

fn process(input: &str) -> String {
    let digit = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = digit.find_iter(line);
        let first = map_str_to_digit(digits.next().unwrap().as_str());
        let last = match digits.last() {
            Some(x) => map_str_to_digit(x.as_str()),
            None => first,
        };

        let number = two_digit_num(first, last);
        sum += number;
    }

    sum.to_string()
}

fn map_str_to_digit(input: &str) -> i32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => input.parse::<i32>().unwrap(),
    }
}

fn two_digit_num(first_digit: i32, second_digit: i32) -> i32 {
    first_digit * 10 + second_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "281"
        );
    }

    #[test]
    fn test_can_map_to_digit() {
        assert_eq!(map_str_to_digit("one"), 1);
        assert_eq!(map_str_to_digit("two"), 2);
        assert_eq!(map_str_to_digit("three"), 3);
        assert_eq!(map_str_to_digit("four"), 4);
        assert_eq!(map_str_to_digit("five"), 5);
        assert_eq!(map_str_to_digit("six"), 6);
        assert_eq!(map_str_to_digit("seven"), 7);
        assert_eq!(map_str_to_digit("eight"), 8);
        assert_eq!(map_str_to_digit("nine"), 9);
        assert_eq!(map_str_to_digit("0"), 0);
        assert_eq!(map_str_to_digit("1"), 1);
        assert_eq!(map_str_to_digit("2"), 2);
        assert_eq!(map_str_to_digit("3"), 3);
        assert_eq!(map_str_to_digit("4"), 4);
        assert_eq!(map_str_to_digit("5"), 5);
        assert_eq!(map_str_to_digit("6"), 6);
        assert_eq!(map_str_to_digit("7"), 7);
        assert_eq!(map_str_to_digit("8"), 8);
        assert_eq!(map_str_to_digit("9"), 9);
    }

    #[test]
    fn test_can_compute_two_digit_num() {
        assert_eq!(two_digit_num(1, 2), 12);
        assert_eq!(two_digit_num(2, 1), 21);
        assert_eq!(two_digit_num(0, 0), 0);
        assert_eq!(two_digit_num(0, 1), 1);
        assert_eq!(two_digit_num(1, 0), 10);
        assert_eq!(two_digit_num(9, 9), 99);
    }
}
