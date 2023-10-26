use std::fs;

fn solve(file: &str) -> String {
    let required_fields = ["usr", "eme", "psw", "age", "loc", "fll"];
    let contents = fs::read_to_string(file).expect("File should exist");
    let mut last_user: String = String::from("");
    let mut valid_users: i32 = 0;

    let users: Vec<&str> = contents.split("\n\n").collect();
    for user in users {
        let clean_user: String = user.replace('\n', " ");
        let user_fields: Vec<&str> = clean_user.split(' ').collect();
        let fields: Vec<&str> = user_fields
            .iter()
            .map(|field| field.split(':').collect::<Vec<&str>>()[0])
            .collect();
        let mut has_req_fields = true;
        for req_field in required_fields {
            match fields.iter().find(|f| f.eq_ignore_ascii_case(req_field)) {
                Some(_) => continue,
                None => {
                    has_req_fields = false;
                    break;
                }
            }
        }
        if has_req_fields {
            valid_users += 1;
            for field in user_fields {
                if field.starts_with("usr") {
                    let username = field.split(":").collect::<Vec<&str>>()[1];
                    last_user = String::from(username);
                }
            }
        }
    }

    return format!("{valid_users}{last_user}");
}

fn main() {
    let result = solve("users.txt");
    println!("{result}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_test() {
        let result = super::solve("test.txt");
        let expected = String::from("3@itziar");
        assert_eq!(result, expected);
    }
}
