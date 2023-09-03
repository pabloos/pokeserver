

use rand::{
    Rng,
    distributions::{Distribution, Standard},
};

pub enum Balance {
    Plus, Minus,
}

impl Distribution<Balance> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Balance {
        match rng.gen_range(0..=1) {
            0 => Balance::Minus,
            _ => Balance::Plus, // 1
        }
    }
}