use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::game;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Serialize)]
struct Save {
    player_name: String,
    class: String,
}

pub fn save_game(rpg: &game::RPG) {
    println!("{}, {}", rpg.player_name.clone(), rpg.class.clone());

    let save: Save = Save {
        player_name: rpg.player_name.clone(),
        class: rpg.class.clone(),
    };

    let toml = toml::to_string(&save).unwrap();

    let path = Path::new("save.toml");

    let mut file = if path.exists() {
        fs::remove_file(path).unwrap();
        File::create(path).unwrap()
    } else {
        File::create(path).unwrap()
    };

    file.write_all(toml.as_bytes()).unwrap();
}
