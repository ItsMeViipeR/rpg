mod game;
pub mod os;
pub mod save_game;

use colored::Colorize;
use std::io;

use crate::game::RPG;

fn main() {
    let mut rpg = RPG::new();

    os::clear();

    println!("Hello voyager! What's your name?");
    let mut player_name = String::new();

    player_name.clear();
    io::stdin().read_line(&mut player_name).unwrap();
    player_name = player_name.replace("\n", "").replace("\r", "");

    os::clear();

    println!(
        "Hello {0}! Let's start the adventure!\n",
        player_name.bold().red()
    );
    println!("Choose a class to start:\n");
    println!("\t1. Knight - The knight has a sword that allows him to kill enemies with style.\n\t2. Wizard - The wizard, with his magical powers can heal himself after fight.");

    let mut class: String = String::new();

    io::stdin().read_line(&mut class).unwrap();
    class = class.replace("\n", "").replace("\r", "");

    rpg.start(player_name, class);
}
