
use crate::core::str::StaticString;

use super::{elements::Element, balance::Balance};

use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub struct Attack {
    name: StaticString,
    element: Element,
    damage: u16,
    margin: u16,
}

impl Attack {
    pub const fn new(name: StaticString, element: Element, damage: u16, margin: u16) -> Self {
        Attack { name, element, damage, margin }
    }

    pub fn name(self) -> StaticString {
        self.name
    }

    pub fn element(self) -> Element {
        self.element
    }

    pub fn damage(&self) -> u16 {
        let mut rng = rand::thread_rng();

        let dmg = rng.gen_range(0..self.margin);

        match rand::random() {
            Balance::Minus => self.damage - dmg,
            Balance::Plus  => self.damage + dmg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Attack;
    use crate::core::domain::elements::Element;

    #[test]
    fn test_damage() {
        let attack = Attack::new("tackle", Element::Fire, 100, 50);

        let dmg = attack.damage();

        if dmg < 50 || dmg > 150 {
            panic!("wrong margin damage")
        }
    }
}