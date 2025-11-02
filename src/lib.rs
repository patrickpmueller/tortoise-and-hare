pub mod betting;
pub mod io;
pub mod race;

pub static RACE_LENGTH: f64 = 1000.0;
pub static HARE_SLEEP_CHANCE: f64 = 0.25;
pub static TORTOISE_SPEED_INTERVAL: (f64, f64) = (5.0, 10.0);
pub static HARE_SPEED_INTERVAL: (f64, f64) = (8.0, 15.0);
pub static FIXED_COSTS: i32 = 20;
pub static INITIAL_BALANCE: i32 = 300;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Animal {
    Hare(Hare),
    Tortoise(Tortoise),
}

#[derive(Debug, PartialEq)]
pub enum RoundResult {
    Winner(Animal),
    Draw,
    InProgress,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hare {
    pub dist: f64,
    pub sleep_chance: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tortoise {
    pub dist: f64,
}

#[derive(Clone, Copy)]
pub struct WinnerTable {
    hare: u32,
    tortoise: u32,
    draw: u32,
}
impl WinnerTable {
    pub const fn new() -> Self {
        WinnerTable {
            hare: 0,
            tortoise: 0,
            draw: 0,
        }
    }

    pub fn update(&mut self, winner: &RoundResult) {
        match winner {
            RoundResult::Draw => self.draw += 1,
            RoundResult::Winner(winner) => match winner {
                Animal::Hare(_) => self.hare += 1,
                Animal::Tortoise(_) => self.hare += 1,
            },
            _ => {}
        }
    }
}

impl Animal {
    pub fn get_animal(&self) -> &str {
        match self {
            Animal::Hare(_) => "hare",
            Animal::Tortoise(_) => "tortoise",
        }
    }

    pub fn get_dist(&self) -> f64 {
        match self {
            Animal::Hare(hare) => hare.dist,
            Animal::Tortoise(tor) => tor.dist,
        }
    }

    pub fn new_animals_vec() -> Vec<Animal> {
        vec![
            Animal::Hare(Hare {
                dist: 0.0,
                sleep_chance: HARE_SLEEP_CHANCE,
            }),
            Animal::Tortoise(Tortoise { dist: 0.0 }),
        ]
    }
}
