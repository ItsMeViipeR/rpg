use crate::{os, save_game};

pub struct RPG {
    pub player_name: String,
    pub class: String,
}

impl RPG {
    pub fn new() -> Self {
        Self {
            player_name: String::new(),
            class: "0".to_owned(),
        }
    }

    pub fn start(&mut self, player_name: String, class: String) {
        os::clear();

        println!("Player name: {} - Class : {}", player_name, class);

        self.player_name = player_name;
        self.class = class;

        save_game::save_game(self);
    }
}
