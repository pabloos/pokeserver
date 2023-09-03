use crate::core::domain::battle::Battle;
use crate::core::domain::pokemon::Pokemon;

pub trait PokemonRepository {
    fn list_pokemons(&self) -> Vec<Pokemon>;
    fn get_pokemon(&self, name: String) -> Option<Pokemon>;
    fn get_random_pokemon(&self) -> Pokemon;
}

pub trait BattleRepository {
    fn save_battle(&mut self, battle: Battle);
    fn delete_battle(&mut self, id: &str);
    fn get_battle(&mut self, id: String) -> Option<&mut Battle>;
}

pub trait Repository: PokemonRepository + BattleRepository {}
impl<T> Repository for T where T: PokemonRepository + BattleRepository {}
