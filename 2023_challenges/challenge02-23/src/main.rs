fn solve(input: &str) -> String {
    let mut value: i32 = 0;
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '#' => value += 1,
            '@' => value -= 1,
            '*' => value = value.pow(2),
            '&' => result.push_str(value.to_string().as_str()),
            _ => (),
        }
    }

    result
}

fn main() {
    let input_path = "./2023_challenges/challenge02-23/input.txt";
    let input_file = std::fs::read_to_string(input_path).expect("Error reading input.txt");
    let input = input_file.as_str();
    let solution = solve(input);
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "##*&";
        let expected = "4";
        let result = solve(input);
        assert_eq!(result, expected);

        let input = "&##&*&@&";
        let expected = "0243";
        let result = solve(input);
        assert_eq!(result, expected);
    }
}
