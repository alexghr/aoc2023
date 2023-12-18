fn main() {
    let input = include_str!("../../input/part2.txt");
    let result = process(input);
    println!("Result (day 2/part 2): {}", result);
}

fn process(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process(""), "");
    }
}
