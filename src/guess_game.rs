// Simple code for learnning Rust
use std::io;
use std::cmp::Ordering;
use rand::random_range;


pub(crate) fn start_game() {
    println!("Starting the game...");
    let answer: u32 = rand_num_create(0, 100); //create random number
    let mut hp: u8 = 10; //chances to guess

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let (result, stop_flag) = check_answer(&guess, &answer);
        hp -= 1;
        println!("{}, Lifes: {}", result, hp);

        if stop_flag {
            println!("Ending the game...");
            break;
        }

        if hp < 1 {
            println!("No more lifes left!");
            println!("You lose!");
            break;
        }
    }
    print!("Game Ended");
}

fn rand_num_create(start: u32, end: u32) -> u32 {
    random_range(start..=end)
}

fn check_answer(input: &u32, answer: &u32) -> (String, bool) {
    match input.cmp(answer) {
        Ordering::Less => {
            (String::from("Too small!"), false)
        }
        Ordering::Greater => {
            (String::from("Too big!"), false)
        }
        Ordering::Equal => {
            (String::from("You win!"), true)
        }
    }
}