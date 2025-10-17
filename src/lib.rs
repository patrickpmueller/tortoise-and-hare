pub mod betting;
pub mod io;
pub mod race;

pub enum Animal {
    Hare(Hare),
    Tortoise(Tortoise),
}

pub struct Hare {
    pub dist: f64,
    pub sleep_chance: f64,
}

pub struct Tortoise {
    pub dist: f64,
}

pub struct Bet {
    pub amount: f64,
    pub on: Animal,
}

pub struct WinnerTable {
    hare: u32,
    tortoise: u32,
    draw: u32,
}
