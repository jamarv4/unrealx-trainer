use super::proc::*;

pub fn init() {
    println!(
        "
    *** UnrealX Trainer ***
    "
    );

    let current_game = find_game();
    println!("Current Game: {:?}", current_game);
}
