use crate::game;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use toml;

#[derive(Deserialize, Serialize)]
struct Save {
    player_name: String,
    class: String,
}

pub fn save_game(rpg: &game::RPG) {
    println!("{}, {}", rpg.player_name, rpg.class);

    let save = Save {
        player_name: rpg.player_name.clone(),
        class: rpg.class.clone(),
    };

    let toml = match toml::to_string(&save) {
        Ok(s) => s,
        Err(e) => {
            panic!("Serialization error: {}", e);
        }
    };

    let path = Path::new("save.toml");

    if path.exists() {
        match fs::remove_file(path) {
            Ok(_) => (),
            Err(e) => {
                panic!("Error removing existing file: {}", e)
            }
        }
    }

    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    match file.write_all(toml.as_bytes()) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
