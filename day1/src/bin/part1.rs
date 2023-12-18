use regex::Regex;

fn main() {
    let input = include_str!("../../input/part1.txt");
    let result = process(input);
    println!("Result (day 1/part 1): {}", result);
}

fn process(input: &str) -> String {
    let digit = Regex::new(r"\d").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = digit.find_iter(line);
        let first = digits.next().unwrap().as_str();
        let last = match digits.last() {
            Some(x) => x.as_str(),
            None => first,
        };

        let number = format!("{first}{last}").parse::<i32>().unwrap();
        sum += number;
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
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            "142"
        );
    }
}
