fn solve(input: Vec<&str>) -> Vec<&str> {
    let mut invalid_keys: Vec<&str> = Vec::new();
    for line in input {
        let (rule, key) = line.split_once(": ").expect("Line with wrong format");
        let (min_max, letter) = rule.split_once(' ').expect("Rule with wrong format");
        let (min, max) = min_max.split_once('-').expect("Rule with wrong format");
        let min: usize = min.parse().expect("Error parsing min");
        let max: usize = max.parse().expect("Error parsing max");
        let mut count = 0;
        for c in key.chars() {
            if c == letter.chars().next().expect("Error getting letter") {
                count += 1;
            }
        }
        if count < min || count > max {
            invalid_keys.push(key);
        }
    }
    invalid_keys
}

fn main() {
    let input_path = "./2023_challenges/challenge03-23/input.txt";
    let input_file = std::fs::read_to_string(input_path).expect("Error reading input.txt");
    let input = input_file.as_str();
    let input: Vec<&str> = input.split('\n').collect();
    let invalid_keys = solve(input);
    let solution = invalid_keys.get(41).expect("Error getting solution");
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = vec!["2-4 f: fgff", "4-5 z: zzzsg", "1-6 h: hhhhhh"];
        let expected = vec!["zzzsg"];
        let result = solve(input);
        assert_eq!(result, expected);
    }
}
