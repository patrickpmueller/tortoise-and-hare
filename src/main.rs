use mini_project::{
    Animal, INITIAL_BALANCE, RoundResult, betting::Bet, io, race, stats::WinnerTable,
};

fn main() {
    io::cli::message("Welcome to The Hare and Tortoise Game!");
    io::cli::sep();

    let mut winners;
    let mut balance: i32;
    if io::cli::get_load_savegame() {
        (winners, balance) = io::savegames::load_game();
    } else {
        winners = WinnerTable::default();
        balance = INITIAL_BALANCE;
    }
    io::cli::sep();

    let mut animals = Animal::new_animals_vec();

    io::cli::message(&format!("You have {balance} carrots."));
    let mut bet = Bet::new_from_input(&animals, balance);
    io::cli::sep();

    loop {
        let result = race::finish::get_result(&animals);
        match result {
            RoundResult::Draw | RoundResult::Winner(_) => {
                io::cli::output_winner(&result);
                balance = bet.next_balance(balance, &result);
                winners.update(&result);

                if balance < 0 {
                    io::cli::sep();
                    io::cli::message(
                        "You do not have enough carrots left and the animals starve.\n\
                        Game Over!",
                    );
                    io::savegames::delete();
                    io::cli::output_stats(&winners);
                    return;
                }

                io::cli::sep();
                if !io::cli::another_race() {
                    io::cli::output_stats(&winners);
                    io::savegames::store_game(winners, balance);
                    io::cli::sep();
                    io::cli::message("Game Saved. Goodbye!");

                    return;
                }
                io::cli::sep();
                io::cli::message(&format!("You have {balance} carrots."));
                bet = Bet::new_from_input(&animals, balance);
            }
            RoundResult::InProgress => {
                animals = race::movement::move_animals(animals);
            }
        }
    }
}
