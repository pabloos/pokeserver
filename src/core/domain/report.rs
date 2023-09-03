use strum_macros::Display;

use super::{attack::Attack, fighter::Fighter};

#[derive(Clone, PartialEq, Display)]
pub enum BattleResult {
    Continue, Win, Lose
}

#[derive(Clone)]
pub struct BattleReport {
    pub id: String,
    pub result: BattleResult,
    pub attacks: Option<(Attack, Attack)>,
    pub fighters: (Fighter, Fighter),
}

impl BattleReport {
    pub fn is_over(self) -> bool {
        self.result == BattleResult::Lose || self.result == BattleResult::Win
    }
}