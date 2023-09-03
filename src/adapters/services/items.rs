

use crate::core::domain::{pokemon::Pokemon, elements::Element, attack::Attack};

type PokemonList = [Pokemon; 4];

pub static POKEMONS: PokemonList = [
    PIKACHU, BULBASUR, CHARMANDER, SQUIRTLE,
];

// pokemons
pub static PIKACHU: Pokemon     = Pokemon::new("Pikachu",    100, Element::Electric, [QUICK, CUT, THUNDER, BALL]);
pub static CHARMANDER: Pokemon  = Pokemon::new("Charmander", 100, Element::Fire,     [TACKLE, CUT, FLAME, SPIN]);
pub static SQUIRTLE: Pokemon    = Pokemon::new("Squirtle",   100, Element::Water,    [TACKLE, QUICK, GUN, RAIN]);
pub static BULBASUR: Pokemon    = Pokemon::new("Bulbasur",   100, Element::Grass,    [TACKLE, QUICK, LEAF, SEED]);

// attacks
pub static TACKLE: Attack   = Attack::new("tackle",     Element::Normal, 15, 1);
pub static CUT: Attack      = Attack::new("cut",        Element::Normal, 20, 10);
pub static QUICK: Attack    = Attack::new("quick_att",  Element::Normal, 22, 5);

pub static THUNDER: Attack  = Attack::new("thunder",    Element::Electric, 20, 10);
pub static BALL: Attack     = Attack::new("elec_ball",  Element::Electric, 15, 1);

pub static GUN: Attack      = Attack::new("water_gun",  Element::Water, 20, 10);
pub static RAIN: Attack     = Attack::new("rain_dance", Element::Water, 15, 1);

pub static FLAME: Attack    = Attack::new("flames",     Element::Fire, 20, 10);
pub static SPIN: Attack     = Attack::new("fire_spin",  Element::Fire, 15, 1);

pub static LEAF: Attack     = Attack::new("leaf_blade", Element::Grass, 20, 10);
pub static SEED: Attack     = Attack::new("seed_bomb",  Element::Grass, 15, 1);