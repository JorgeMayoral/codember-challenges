fn check_password(password: i32) -> bool {
    let string_pass = password.to_string();
    let fives_count = string_pass.chars().filter(|c| c == &'5').count();
    let is_five_repeated = fives_count >= 2;
    let mut is_increasing = true;
    let mut prev_number = 0;
    for num in string_pass.split("") {
        let number = String::from(num).parse::<i32>();
        if let Ok(n) = number {
            if n < prev_number {
                is_increasing = false;
                break;
            }
            prev_number = n;
        }
    }
    is_five_repeated && is_increasing
}

fn main() {
    let mut passwords: Vec<i32> = vec![];
    for i in 11098..=98123 {
        if check_password(i) {
            passwords.insert(passwords.len(), i);
        }
    }
    println!("{}-{}", passwords.len(), passwords[55]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_password_test_1() {
        let result = super::check_password(55678);
        assert!(result);
    }

    #[test]
    fn check_password_test_2() {
        let result = super::check_password(12555);
        assert!(result);
    }

    #[test]
    fn check_password_test_3() {
        let result = super::check_password(55555);
        assert!(result);
    }

    #[test]
    fn check_password_test_4() {
        let result = super::check_password(12345);
        assert!(!result);
    }

    #[test]
    fn check_password_test_5() {
        let result = super::check_password(57775);
        assert!(!result);
    }
}
