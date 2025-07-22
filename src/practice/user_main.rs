//! Main file of User application.

mod user_info;

use std::io;
use user_info::user_info::User;

fn main() {
    let mut user_list: Vec<User> = Vec::new();

     while guess.to_lowercase() == "end" || guess.to_lowercase() = "exit" {
         let mut guess = String::new();
         io::stdin()
             .read_line(&mut guess)
             .expect("Failed to read line");

    }
}