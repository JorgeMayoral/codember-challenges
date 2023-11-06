use std::collections::HashMap;

fn solve(input: Vec<&str>) -> String {
    let input_lowercase: Vec<String> = input.iter().map(|word| word.to_lowercase()).collect();
    let mut words_order: Vec<&String> = Vec::new();
    let mut words: HashMap<&String, u32> = HashMap::new();
    input_lowercase.iter().for_each(|word| {
        if !words.contains_key(word) {
            words.insert(word, 1);
            words_order.push(word);
        } else {
            let count = words.get(word).expect("Error getting word count");
            words.insert(word, count + 1);
        }
    });
    let mut message = String::new();
    for word in words_order {
        let count = words.get(word).expect("Error getting word count");
        message.push_str(&format!("{word}{count}"));
    }
    message
}

fn main() {
    let input_path = "./2023_challenges/challenge01-23/text.txt";
    let input_file = std::fs::read_to_string(input_path).expect("Error reading text.txt");
    let input: Vec<&str> = input_file.split_whitespace().collect();
    let message = solve(input);
    println!("{message}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = vec!["llaveS", "casa", "CASA", "casa", "llaves"];
        let expected = "llaves2casa3";
        let actual = solve(input);
        assert_eq!(actual, expected);

        let input = vec!["taza", "ta", "za", "taza"];
        let expected = "taza2ta1za1";
        let actual = solve(input);
        assert_eq!(actual, expected);

        let input = vec!["casas", "casa", "casasas"];
        let expected = "casas1casa1casasas1";
        let actual = solve(input);
        assert_eq!(actual, expected);
    }
}
