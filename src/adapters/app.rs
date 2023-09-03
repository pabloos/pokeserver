use std::sync::Mutex;

use crate::core::{
    domain::{battle::Battle, pokemon::Pokemon, report::BattleReport},
    ports::{app::PokeApp, services::Repository},
};

pub struct App {
    repo: Box<Mutex<dyn Repository + Send + Sync + 'static>>,
}

pub fn new_pokeapp(repo: Box<Mutex<dyn Repository + Send + Sync>>) -> App {
    App { repo }
}

impl PokeApp for App {
    fn list_pokemons(&self) -> Vec<Pokemon> {
        self.repo.lock().unwrap().list_pokemons()
    }

    fn get_pokemon(&self, name: String) -> Option<Pokemon> {
        self.repo.lock().unwrap().get_pokemon(name)
    }

    fn create_battle(&self, pokemon_name: String) -> BattleReport {
        let mut repo = self.repo.lock().unwrap();

        let player = repo.get_pokemon(pokemon_name).unwrap();
        let opp = repo.get_random_pokemon();

        let battle = Battle::new_battle(player, opp);

        let report = battle.start();

        repo.save_battle(battle);

        report
    }

    fn continue_battle(&self, battle_id: String, attack_name: String) -> BattleReport {
        let mut repo = self.repo.lock().unwrap();

        let battle = repo.get_battle(battle_id).unwrap();
        let attack = battle.search_player_attack(attack_name).unwrap();

        battle.round(attack)
    }
}
