use std::collections::HashMap;

fn main() {
    let input_file = std::fs::read_to_string("text.txt").expect("Error reading text.txt");
    let input: Vec<&str> = input_file.split_whitespace().collect();
    let mut words_order: Vec<&str> = Vec::new();
    let mut words: HashMap<&str, u32> = HashMap::new();
    input.iter().for_each(|word| {
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
    println!("{message}")
}
