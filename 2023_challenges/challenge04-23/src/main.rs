fn solve(input: &str) -> bool {
    let (filename, checksum) = input.split_once('-').expect("Invalid input format");
    let mut letter_counts: Vec<(char, usize)> = vec![];
    for letter in filename.chars() {
        match letter_counts.iter().position(|l| l.0 == letter) {
            Some(index) => {
                letter_counts[index].1 += 1;
            }
            None => {
                letter_counts.push((letter, 1));
            }
        }
    }
    let unique_letters: Vec<&(char, usize)> = letter_counts.iter().filter(|l| l.1 == 1).collect();
    let unique_letters: Vec<String> = unique_letters.iter().map(|l| l.0.to_string()).collect();
    let calculated_checksum = unique_letters.join("");
    calculated_checksum == checksum
}

fn main() {
    let input_path = "./2023_challenges/challenge04-23/input.txt";
    let input_file = std::fs::read_to_string(input_path).expect("Error reading input.txt");
    let input = input_file.as_str();
    let lines = input.lines();
    let mut valid_lines: Vec<&str> = vec![];
    for line in lines {
        let solution = solve(line);
        if solution {
            valid_lines.push(line);
        }
    }
    let file_33 = valid_lines.get(32).expect("Element 33 should exist");
    println!("{file_33}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct() {
        let input = "xyzz33-xy";
        let expected = true;
        let result = solve(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_incorrect_count() {
        let input = "abcca1-ab1";
        let expected = false;
        let result = solve(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_incorrect_order() {
        let input = "abbc11-ca";
        let expected = false;
        let result = solve(input);
        assert_eq!(result, expected);
    }
}
