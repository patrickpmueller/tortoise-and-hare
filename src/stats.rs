use crate::{Animal, RoundResult};

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct WinnerTable {
    pub hare: u32,
    pub tortoise: u32,
    pub draw: u32,
}

impl WinnerTable {
    pub fn update(&mut self, winner: &RoundResult) {
        match winner {
            RoundResult::Draw => self.draw += 1,
            RoundResult::Winner(winner) => match winner {
                Animal::Hare(_) => self.hare += 1,
                Animal::Tortoise(_) => self.tortoise += 1,
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner_table_update() {
        let mut table = WinnerTable::default();
        let hare = Animal::Hare(crate::Hare {
            dist: 0.0,
            sleep_chance: 0.0,
        });
        let tortoise = Animal::Tortoise(crate::Tortoise { dist: 0.0 });

        table.update(&RoundResult::Draw);
        assert_eq!(
            table,
            WinnerTable {
                hare: 0,
                tortoise: 0,
                draw: 1,
            }
        );

        table.update(&RoundResult::InProgress);
        assert_eq!(
            table,
            WinnerTable {
                hare: 0,
                tortoise: 0,
                draw: 1,
            }
        );

        table.update(&RoundResult::Winner(hare));
        assert_eq!(
            table,
            WinnerTable {
                hare: 1,
                tortoise: 0,
                draw: 1,
            }
        );

        table.update(&RoundResult::Winner(tortoise));
        assert_eq!(
            table,
            WinnerTable {
                hare: 1,
                tortoise: 1,
                draw: 1,
            }
        );

        table.update(&RoundResult::Winner(tortoise));
        assert_eq!(
            table,
            WinnerTable {
                hare: 1,
                tortoise: 2,
                draw: 1,
            }
        );
    }
}
