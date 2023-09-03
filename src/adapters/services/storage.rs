use rand::seq::SliceRandom;

use std::{collections::HashMap, sync::Mutex};

use crate::core::{
    domain::{battle::Battle, pokemon::Pokemon},
    ports::services::{BattleRepository, PokemonRepository},
};

use super::items::POKEMONS;

pub struct Storage {
    battles: HashMap<String, Battle>,
}

pub fn new_pokemon_storage() -> Box<Mutex<Storage>> {
    Box::new(Mutex::new(Storage {
        battles: HashMap::new(),
    }))
}

impl PokemonRepository for Storage {
    fn list_pokemons(&self) -> Vec<Pokemon> {
        POKEMONS.to_vec()
    }

    fn get_pokemon(&self, name: String) -> Option<Pokemon> {
        POKEMONS.into_iter().find(|pokemon| pokemon.name().to_ascii_lowercase() == name)
    }

    fn get_random_pokemon(&self) -> Pokemon {
        *POKEMONS.to_vec().choose(&mut rand::thread_rng()).unwrap()
    }
}

impl BattleRepository for Storage {
    fn get_battle(&mut self, gym_name: String) -> Option<&mut Battle> {
        self.battles.get_mut(gym_name.as_str())
    }

    fn save_battle(&mut self, battle: Battle) {
        self.battles.insert(battle.id(), battle);
    }

    fn delete_battle(self: &mut Storage, gym_name: &str) {
        self.battles.remove(gym_name);
    }
}
