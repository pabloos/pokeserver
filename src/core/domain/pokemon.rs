use rand::seq::IteratorRandom;

use super::attack::Attack;
use super::elements::Element;
use crate::core::str::StaticString;

#[derive(Clone, Copy)]
pub struct Pokemon {
    name: StaticString,
    life: u16,
    element: Element,
    attacks: [Attack; 4],
}

impl Pokemon {
    pub const fn new(name: StaticString, life: u16, element: Element, attacks: [Attack; 4]) -> Pokemon {
        Pokemon { name, life, element, attacks }
    }

    pub fn find_attack(self, name: String) -> Option<Attack> {
        self.attacks.into_iter().find(|attack| attack.name() == name)
    }

    pub fn name(&self) -> StaticString {
        self.name
    }

    pub fn life(&self) -> u16 {
        self.life
    }

    pub fn element(&self) -> Element {
        self.element
    }

    pub fn attacks(&self) -> &[Attack] {
        &self.attacks
    }

    pub fn weak_to(self, element: Element) -> bool {
        self.element.weak_to(element)
    }

    pub fn strong_to(self, element: Element) -> bool {
        self.element.strong_to(element)
    }

    pub fn random_attack(self) -> Option<Attack> {
        let i = (0..self.attacks.len()).choose(&mut rand::thread_rng())?;

        Some(self.attacks[i])
    }
}