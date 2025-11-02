use std::io::{self, Write};

use crate::{Animal, RoundResult, WinnerTable};

pub fn another_race() -> bool {
    input_bool("Do you want to start another race [Y/n]? ")
}

pub fn get_load_savegame() -> bool {
    input_bool("Do you want to load a savegame [Y/n]? ")
}

fn input_bool(msg: &str) -> bool {
    print!("{msg}");
    io::stdout().flush().expect("Error when flushing stdout.");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error when reading from stdin.");

    match buffer.trim().to_lowercase().as_str() {
        "y" | "yes" | "" => true,
        "n" | "no" => false,
        _ => input_bool(msg),
    }
}

pub fn sep() {
    println!();
}

pub fn message(msg: &str) {
    println!("{msg}");
}

pub fn output_stats(table: &WinnerTable) {
    println!(
        "The hare has won {} times, the tortoise has won {} times and there were {} ties.",
        table.hare, table.tortoise, table.draw
    );
}

pub fn output_winner(result: &RoundResult) {
    match result {
        RoundResult::Draw => println!(
            "The hare and the tortoise went through the finish line together. The race ends in a draw!"
        ),
        RoundResult::Winner(w) => match w {
            Animal::Hare(_) => println!("The hare won through its incredible speed!"),
            Animal::Tortoise(_) => {
                println!("The tortoise won by taking the race slow and steady!")
            }
        },
        RoundResult::InProgress => {}
    }
}

// Takes hare in pos 0 and tortoise in pos 1
pub fn get_bet_animal(animals: &Vec<Animal>) -> Animal {
    print!(
        "Which of the following animals do you want to bet on?\n\
        The available options are: Hare and Tortoise\n\
        Enter your chosen animal: ",
    );
    io::stdout().flush().expect("Error when flushing stdout.");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error when reading from stdin.");

    if let Some(animal) = validate_animal(&buffer, animals) {
        animal
    } else {
        get_bet_animal(animals)
    }
}

pub fn get_bet_amount(animal_bet_on: Animal, balance: i32) -> i32 {
    print!(
        "How many carrots do you want to bet on the {}? ",
        animal_bet_on.get_animal()
    );
    io::stdout().flush().expect("Error when flushing stdout.");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error when reading from stdin.");

    match validate_bet_amount(&buffer, balance) {
        Some(n) => n,
        None => get_bet_amount(animal_bet_on, balance),
    }
}

fn validate_bet_amount(inp: &str, balance: i32) -> Option<i32> {
    match inp.trim().parse() {
        Ok(n) if n <= balance && n >= 0 => Some(n),
        _ => None,
    }
}

fn validate_animal(inp: &str, animals: &Vec<Animal>) -> Option<Animal> {
    assert_eq!(animals[0].get_animal(), "hare");
    assert_eq!(animals[1].get_animal(), "tortoise");

    match inp.trim().to_lowercase().as_str() {
        "hare" | "h" | "1" => Some(animals[0]),
        "tortoise" | "t" | "2" => Some(animals[1]),
        _ => None,
    }
}
