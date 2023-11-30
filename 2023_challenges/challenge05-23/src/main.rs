fn is_valid_id(id: &str) -> bool {
    id.chars().all(|c| c.is_ascii_alphanumeric())
}

fn is_valid_username(username: &str) -> bool {
    username.chars().all(|c| c.is_ascii_alphanumeric())
}

fn is_valid_email(email: &str) -> bool {
    let email_parts = email.split('@').collect::<Vec<&str>>();
    if email_parts.len() != 2 {
        return false;
    }
    let [username, domain] = [email_parts[0], email_parts[1]];
    if username.is_empty() || domain.is_empty() {
        return false;
    }
    let domain_parts = domain.split('.').collect::<Vec<&str>>();
    if domain_parts.len() != 2 {
        return false;
    }
    let [domain_name, domain_extension] = [domain_parts[0], domain_parts[1]];
    if domain_name.is_empty() || domain_extension.is_empty() {
        return false;
    }
    if username.matches('@').count() > 1 || domain.matches('.').count() > 1 {
        return false;
    }
    true
}

fn is_valid_age(age: &str) -> bool {
    age.is_empty() || age.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_location(location: &str) -> bool {
    location.is_empty()
        || location
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c.is_whitespace())
}

fn solve(input: &str) -> (bool, &str) {
    let fields = input.splitn(5, ',').collect::<Vec<&str>>();
    let fields: [&str; 5] = [fields[0], fields[1], fields[2], fields[3], fields[4]];
    let [id, username, email, age, location] = fields;
    (
        is_valid_id(id)
            && is_valid_username(username)
            && is_valid_email(email)
            && is_valid_age(age)
            && is_valid_location(location),
        username,
    )
}

fn main() {
    let input_path = "./2023_challenges/challenge05-23/input.txt";
    let input_file = std::fs::read_to_string(input_path).expect("Error reading input.txt");
    let input = input_file.as_str();
    let lines = input.lines();
    let mut invalid_usernames = Vec::new();
    for line in lines {
        let (valid, username) = solve(line);
        if !valid {
            invalid_usernames.push(username);
            println!("{line}");
        }
    }
    let invaid_first_letter: Vec<String> = invalid_usernames
        .iter()
        .map(|s| s[0..1].to_string())
        .collect();
    let secret_message = invaid_first_letter.join("");
    println!("{}", secret_message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_user() {
        let input = "1a421fa,alex,alex9@gmail.com,18,Barcelona";
        let expected_result = true;
        let expected_username = "alex";
        let result = solve(input);
        assert_eq!(result.0, expected_result);
        assert_eq!(result.1, expected_username);

        let input = "494ee0,madeval,mdv@twitch.tv,,";
        let expected_result = true;
        let expected_username = "madeval";
        let result = solve(input);
        assert_eq!(result.0, expected_result);
        assert_eq!(result.1, expected_username);
    }

    #[test]
    fn test_invalid_id() {
        let input = "9412p_m,maria,mb@hotmail.com,22,CDMX";
        let expected_result = false;
        let expected_username = "maria";
        let result = solve(input);
        assert_eq!(result.0, expected_result);
        assert_eq!(result.1, expected_username);
    }

    #[test]
    fn test_invalid_email() {
        let input = "494ee0,madeval,twitch.tv,22,Montevideo";
        let expected_result = false;
        let expected_username = "madeval";
        let result = solve(input);
        assert_eq!(result.0, expected_result);
        assert_eq!(result.1, expected_username);
    }
}
