use crate::{Animal, RoundResult};

pub fn get_result(animals: &Vec<Animal>) -> RoundResult {
    let mut winners: Vec<&Animal> = vec![];
    for animal in animals {
        if animal.get_dist() >= 1000.0 {
            winners.push(animal);
        }
    }

    if winners.len() > 1 {
        return RoundResult::Draw;
    }

    match winners.get(0) {
        Some(winner) => RoundResult::Winner(**winner),
        None => RoundResult::InProgress,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner() {
        let hare_zero = Animal::Hare(crate::Hare {
            dist: 0.0,
            sleep_chance: crate::HARE_SLEEP_CHANCE,
        });
        let hare_not_finished = Animal::Hare(crate::Hare {
            dist: 2.3,
            sleep_chance: crate::HARE_SLEEP_CHANCE,
        });
        let hare_at_finish = Animal::Hare(crate::Hare {
            dist: crate::RACE_LENGTH,
            sleep_chance: crate::HARE_SLEEP_CHANCE,
        });
        let hare_past_finish = Animal::Hare(crate::Hare {
            dist: crate::RACE_LENGTH + 3.6,
            sleep_chance: crate::HARE_SLEEP_CHANCE,
        });

        let tortoise_zero = Animal::Tortoise(crate::Tortoise { dist: 0.0 });
        let tortoise_not_finished = Animal::Tortoise(crate::Tortoise { dist: 1.7 });
        let tortoise_at_finish = Animal::Tortoise(crate::Tortoise {
            dist: crate::RACE_LENGTH,
        });
        let tortoise_past_finish = Animal::Tortoise(crate::Tortoise {
            dist: crate::RACE_LENGTH + 3.6,
        });

        assert_eq!(
            get_result(&vec![hare_zero, tortoise_zero]),
            RoundResult::InProgress
        );
        assert_eq!(
            get_result(&vec![hare_zero, tortoise_not_finished]),
            RoundResult::InProgress
        );
        assert_eq!(
            get_result(&vec![hare_not_finished, tortoise_not_finished]),
            RoundResult::InProgress
        );
        assert_eq!(
            get_result(&vec![hare_zero, tortoise_at_finish]),
            RoundResult::Winner(tortoise_at_finish)
        );
        assert_eq!(
            get_result(&vec![hare_not_finished, tortoise_past_finish]),
            RoundResult::Winner(tortoise_past_finish)
        );
        assert_eq!(
            get_result(&vec![hare_at_finish, tortoise_past_finish]),
            RoundResult::Draw
        );
        assert_eq!(
            get_result(&vec![hare_at_finish, tortoise_at_finish]),
            RoundResult::Draw
        );
        assert_eq!(
            get_result(&vec![hare_past_finish, tortoise_past_finish]),
            RoundResult::Draw
        );
        assert_eq!(
            get_result(&vec![hare_at_finish, tortoise_not_finished]),
            RoundResult::Winner(hare_at_finish)
        );
    }
}
