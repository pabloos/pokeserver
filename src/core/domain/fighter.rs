
use super::{pokemon::Pokemon, attack::Attack};

#[derive(Clone, Copy)]
pub struct Fighter {
    pub pokemon: Pokemon,
    pub life: u16,
}

impl Fighter {
    pub fn new_fighter(pokemon: Pokemon) -> Self {
        Fighter {
            pokemon,
            life: pokemon.life(),
        }
    }

    pub fn random_attack(&self) -> Option<Attack> {
        self.pokemon.random_attack()
    }

    pub fn receive(&mut self, attack: Attack) {
        let element = attack.element();

        let damage = if self.pokemon.weak_to(element) {
            attack.damage() * 2
        } else if self.pokemon.strong_to(element) {
            attack.damage() / 2
        } else {
            attack.damage()
        };

        self.life = if damage > self.life { 0 } else { self.life - damage };
    }

    pub fn find_attack(&self, attack_name: String) -> Option<Attack> {
        self.pokemon.find_attack(attack_name)
    }
}