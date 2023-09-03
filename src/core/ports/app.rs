

use crate::core::domain::{pokemon::Pokemon, report::BattleReport};

pub trait PokeApp {
    fn list_pokemons(&self) -> Vec<Pokemon>;
    fn get_pokemon(&self, name: String) -> Option<Pokemon>;

    fn create_battle(&self, pokemon_name: String) -> BattleReport;
    fn continue_battle(&self, battle_id: String, attack_name: String) -> BattleReport;
}