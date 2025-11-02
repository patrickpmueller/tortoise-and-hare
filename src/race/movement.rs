use crate::Animal;

pub fn move_animals(animals: Vec<Animal>) -> Vec<Animal> {
    let mut animals = animals;
    for animal in &mut animals {
        match animal {
            Animal::Hare(hare) => {
                if fastrand::f64() >= hare.sleep_chance {
                    let d_dist = crate::HARE_SPEED_INTERVAL.0
                        + (crate::HARE_SPEED_INTERVAL.1 - crate::HARE_SPEED_INTERVAL.0)
                            * fastrand::f64();
                    hare.dist += d_dist;
                }
            }
            Animal::Tortoise(tortoise) => {
                let d_dist = crate::TORTOISE_SPEED_INTERVAL.0
                    + (crate::TORTOISE_SPEED_INTERVAL.1 - crate::TORTOISE_SPEED_INTERVAL.0)
                        * fastrand::f64();
                tortoise.dist += d_dist;
            }
        }
    }

    animals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_animals() {
        let hare_zero_no_sleep = Animal::Hare(crate::Hare {
            dist: 0.0,
            sleep_chance: 0.0,
        });
        let hare_twenty_no_sleep = Animal::Hare(crate::Hare {
            dist: 20.0,
            sleep_chance: 0.0,
        });
        let hare_twenty_all_sleep = Animal::Hare(crate::Hare {
            dist: 20.0,
            sleep_chance: 1.0,
        });
        let hare_zero_all_sleep = Animal::Hare(crate::Hare {
            dist: 0.0,
            sleep_chance: 1.0,
        });

        let tortoise_zero = Animal::Tortoise(crate::Tortoise { dist: 0.0 });
        let tortoise_twenty = Animal::Tortoise(crate::Tortoise { dist: 20.0 });

        let mut race = vec![hare_zero_all_sleep];
        race = move_animals(race);
        race = move_animals(race);
        race = move_animals(race);
        race = move_animals(race);
        race = move_animals(race);
        assert_eq!(race, vec![hare_zero_all_sleep]);

        let mut race = vec![hare_twenty_all_sleep];
        race = move_animals(race);
        race = move_animals(race);
        race = move_animals(race);
        race = move_animals(race);
        race = move_animals(race);
        assert_eq!(race, vec![hare_twenty_all_sleep]);

        let mut race = vec![hare_twenty_no_sleep];
        race = move_animals(race);
        race = move_animals(race);
        assert!(race[0].get_dist() <= 20.0 + crate::HARE_SPEED_INTERVAL.1 * 2.0);
        assert!(race[0].get_dist() >= 20.0 + crate::HARE_SPEED_INTERVAL.0 * 2.0);

        let mut race = vec![hare_zero_no_sleep];
        race = move_animals(race);
        race = move_animals(race);
        assert!(race[0].get_dist() <= 0.0 + crate::HARE_SPEED_INTERVAL.1 * 2.0);
        assert!(race[0].get_dist() >= 0.0 + crate::HARE_SPEED_INTERVAL.0 * 2.0);

        let mut race = vec![tortoise_twenty];
        race = move_animals(race);
        race = move_animals(race);
        assert!(race[0].get_dist() <= 20.0 + crate::TORTOISE_SPEED_INTERVAL.1 * 2.0);
        assert!(race[0].get_dist() >= 20.0 + crate::TORTOISE_SPEED_INTERVAL.0 * 2.0);

        let mut race = vec![tortoise_zero];
        race = move_animals(race);
        race = move_animals(race);
        assert!(race[0].get_dist() <= 0.0 + crate::TORTOISE_SPEED_INTERVAL.1 * 2.0);
        assert!(race[0].get_dist() >= 0.0 + crate::TORTOISE_SPEED_INTERVAL.0 * 2.0);
    }
}
