use std::io::prelude::*;
use std::{fs::File, path::Path};

use sha3::{Digest, Sha3_256};

use crate::WinnerTable;

pub fn delete() {
    let path = Path::new("t_and_h_savegame.csv");
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("Error: Couldn't create  file {}: {}", display, why),
        Ok(file) => file,
    };

    if let Err(why) = file.write_all(b"") {
        panic!("Error: Couldn't write to {}: {}", display, why);
    }
}

pub fn store_game(stats: WinnerTable, balance: i32) {
    let data = format!(
        "{},{},{},{}",
        stats.hare, stats.tortoise, stats.draw, balance
    );
    let mut hasher = Sha3_256::new();
    hasher.update(format!("{}.salted!{}", data, "0211065ebe237db4dee6e8dcbaa5db74").as_bytes());
    let hash = hasher.finalize();

    let path = Path::new("t_and_h_savegame.csv");
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("Error: Couldn't create  file {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(format!("{},{}", data, hex::encode(hash)).as_bytes()) {
        Err(why) => panic!("Error: Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully saved game in the file {}", display),
    }
}

pub fn load_game() -> (WinnerTable, i32) {
    let path = Path::new("t_and_h_savegame.csv");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("Error: Couldn't open file {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buf = String::new();
    if let Err(why) = file.read_to_string(&mut buf) {
        panic!("Error: Couldn't read file {}: {}", display, why);
    }

    let data: Vec<&str> = buf.split(",").collect();

    let mut hasher = Sha3_256::new();
    hasher.update(
        format!(
            "{},{},{},{}.salted!{}",
            data[0], data[1], data[2], data[3], "0211065ebe237db4dee6e8dcbaa5db74"
        )
        .as_bytes(),
    );
    let hash = hasher.finalize();

    let corr_error = format!("Couldn't load savegame {}: data corrputed", display);
    if hex::encode(hash) == data[4] {
        return (
            WinnerTable {
                hare: data[0].parse().expect(&corr_error),
                tortoise: data[1].parse().expect(&corr_error),
                draw: data[2].parse().expect(&corr_error),
            },
            data[3].parse().expect(&corr_error),
        );
    }
    panic!("{}", &corr_error);
}
