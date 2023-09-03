use nanoid::nanoid;

use super::{
    attack::Attack,
    fighter::Fighter,
    pokemon::Pokemon,
    report::{BattleReport, BattleResult},
};

pub struct Battle {
    id: String,
    player: Fighter,
    opp: Fighter,
}

impl Battle {
    pub fn new_battle(player_pokemon: Pokemon, opp_pokemon: Pokemon) -> Self {
        Battle {
            id: nanoid!(),
            player: Fighter::new_fighter(player_pokemon),
            opp: Fighter::new_fighter(opp_pokemon),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn start(&self) -> BattleReport {
        let lifes = self.lifes();

        BattleReport {
            id: self.id.to_string(),
            fighters: (self.player, self.opp),
            attacks: None,
            result: match lifes {
                (0, _) => BattleResult::Lose,
                (_, 0) => BattleResult::Win,
                _ => BattleResult::Continue,
            },
        }
    }

    pub fn round(&mut self, attack: Attack) -> BattleReport {
        self.opp.receive(attack);

        let counter = self.opp.random_attack().unwrap();
        self.player.receive(counter);

        BattleReport {
            id: self.id.to_string(),
            fighters: (self.player, self.opp),
            attacks: Some((attack, counter)),
            result: match self.lifes() {
                (0, _) => BattleResult::Lose,
                (_, 0) => BattleResult::Win,
                _ => BattleResult::Continue,
            },
        }
    }

    fn lifes(&self) -> (u16, u16) {
        (self.player.life, self.opp.life)
    }

    pub fn search_player_attack(&self, attack_name: String) -> Option<Attack> {
        self.player.find_attack(attack_name)
    }
}
