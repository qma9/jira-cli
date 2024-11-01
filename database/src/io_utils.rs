use std::io;

#[allow(dead_code)]
pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}
