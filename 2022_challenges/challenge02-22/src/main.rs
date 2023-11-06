const MIN_CHAR: u32 = 97;
const MAX_CHAR: u32 = 122;

fn compare_num(num: u32) -> bool {
    (MIN_CHAR..=MAX_CHAR).contains(&num)
}

fn split_chars(char_str: &str) -> Vec<u32> {
    let mut chars_vec: Vec<u32> = vec![];
    let mut new_char: u32 = 0;
    for c in char_str.split("").collect::<Vec<&str>>() {
        if c.is_empty() {
            continue;
        }
        match c.parse::<u32>() {
            Ok(n) => {
                new_char = new_char * 10 + n;
                if compare_num(new_char) {
                    chars_vec.insert(chars_vec.len(), new_char);
                    new_char = 0;
                } else {
                    continue;
                }
            }
            Err(_) => panic!("Input should contain only numbers, recived: \"{char_str}\""),
        }
    }

    chars_vec
}

fn solve(input: &str) -> String {
    let words: Vec<&str> = input.split(' ').collect();
    let mut decrypted_sentence: Vec<String> = vec![];
    for word in words {
        let chars_vec = split_chars(word);
        let mut decrypted_word: String = String::from("");
        for character in chars_vec {
            let character = char::from_u32(character);
            match character {
                Some(c) => decrypted_word.insert(decrypted_word.len(), c),
                None => panic!("This should not happen"),
            }
        }
        decrypted_sentence.insert(decrypted_sentence.len(), decrypted_word);
    }

    decrypted_sentence.join(" ")
}

fn main() {
    let result = solve("11610497110107115 102111114 11210897121105110103 9911110010110998101114 11210810197115101 11510497114101");
    println!("{result}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_test() {
        assert_eq!(super::solve("109105100117"), String::from("midu"));
        assert_eq!(
            super::solve("9911110010110998101114"),
            String::from("codember")
        );
        assert_eq!(
            super::solve("9911110010110998101114 109105100117"),
            String::from("codember midu")
        );
        assert_eq!(
            super::solve("11210897121 116101116114105115"),
            String::from("play tetris")
        );
    }
}
