use std::collections::HashMap;
use lycan::shared::gamestate::{Gamestate, Player, UpdateResponse};

pub struct ClientGamestate {
    pub gamestate: Gamestate,
    pub player_id: Option<String>,
    pub game_id: Option<String>,
}

impl ClientGamestate {
    pub fn load(_string: String) -> ClientGamestate {
        ClientGamestate {
            gamestate: Gamestate::default(),
            player_id: None,
            game_id: None,
        }
    }

    pub fn get_player(&self) -> Option<&Player> {
        match &self.player_id {
            Some(player_id) => Some(self.gamestate.players.get(player_id)?),
            None => None,
        }
    }

    pub fn get_mut_player(&mut self) -> Option<&mut Player> {
        match &self.player_id {
            Some(player_id) => Some(self.gamestate.players.get_mut(player_id)?),
            None => None,
        }
    }

    pub fn set_player(&mut self, player_id: String) {
        self.gamestate.players.insert(player_id.clone(), Player{name: String::from("foo"), position: (0.0, 0.0)});
        self.player_id = Some(player_id);
    }

    pub fn get_players(&self) -> &HashMap<String, Player> {
        &self.gamestate.players
    }

    pub fn set_game(&mut self, game_id: String) {
        self.game_id = Some(game_id);
    }

    pub fn get_game_id(&self) -> String {
        match &self.game_id {
            Some(game_id) => game_id.clone(),
            None => String::from(""),
        }
    }

    pub fn get_player_id(&self) -> String {
        match &self.player_id {
            Some(player_id) => player_id.clone(),
            None => String::from(""),
        }
    }

    pub fn update(&mut self, data: UpdateResponse) {
        for (player_id, player_state) in data.players {
            match self.gamestate.players.get_mut(&player_id) {
                Some(player) => {
                    player.position = player_state.position;
                },
                None => {
                    self.gamestate.players.insert(player_id, Player{position: player_state.position, name: player_state.name});
                },
            }
        }
    }
}
