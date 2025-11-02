use crate::{Animal, RoundResult, io};

#[derive(Clone, Copy)]
pub struct Bet {
    pub amount: i32,
    pub on: Animal,
}

impl Bet {
    pub fn new_from_input(animals: &Vec<Animal>, balance: i32) -> Bet {
        let on = io::cli::get_bet_animal(animals);
        let amount = io::cli::get_bet_amount(on, balance);
        Bet { on, amount }
    }

    pub fn next_balance(&self, balance: i32, result: &RoundResult) -> i32 {
        let mut balance = balance;
        match result {
            RoundResult::Draw => {
                balance -= self.amount;
                balance -= crate::FIXED_COSTS;
                io::cli::message(&format!(
                    "You have lost your bet and have to give {} carrots away.",
                    self.amount
                ));
            }
            RoundResult::Winner(winner) => {
                if &self.on == winner {
                    balance += self.amount;
                    io::cli::message(&format!(
                        "You have won your bet and get {} more carrots.",
                        self.amount
                    ));
                } else {
                    balance -= self.amount;
                    io::cli::message(&format!(
                        "You have lost your bet and have to give {} carrots away.",
                        self.amount
                    ));
                }

                io::cli::message(&format!(
                    "The animals are hungry need to eat. You give them {} carrots.",
                    crate::FIXED_COSTS
                ));
                balance -= crate::FIXED_COSTS;
            }
            RoundResult::InProgress => {}
        }
        balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_balance() {
        let hare = Animal::Hare(crate::Hare {
            dist: 0.0,
            sleep_chance: 0.0,
        });
        let tortoise = Animal::Tortoise(crate::Tortoise { dist: 0.0 });

        assert_eq!(
            Bet {
                on: hare,
                amount: 10.0
            }
            .next_balance(100.0, &RoundResult::Winner(hare)),
            110.0
        );
        assert_eq!(
            Bet {
                on: tortoise,
                amount: 10.0
            }
            .next_balance(100.0, &RoundResult::Winner(hare)),
            90.0
        );
        assert_eq!(
            Bet {
                on: hare,
                amount: 50.0
            }
            .next_balance(200.0, &RoundResult::Winner(tortoise)),
            150.0
        );
        assert_eq!(
            Bet {
                on: tortoise,
                amount: 50.0
            }
            .next_balance(200.0, &RoundResult::Winner(tortoise)),
            250.0
        );
    }
}
