use serde::Serialize;
use utoipa::ToSchema;

use crate::core::{
    domain::{
        pokemon::Pokemon,
        report::BattleReport,
    },
    str::StaticString,
};

#[derive(Serialize, ToSchema)]
pub struct PokemonRes {
    name: StaticString,
    life: u16,
    element: String,
    url: String,
    fight: String,
}

impl From<Pokemon> for PokemonRes {
    fn from(pokemon: Pokemon) -> Self {
        PokemonRes {
            name: pokemon.name(),
            life: pokemon.life(),
            element: pokemon.element().to_string(),
            url: format!("/pokedex/{}", pokemon.name().to_ascii_lowercase()),
            fight: format!("/pokedex/{}/fight", pokemon.name().to_ascii_lowercase()),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Battle {
    report: Report,
    attack_options: Vec<AttackOption>,
}

#[derive(Serialize, ToSchema)]
pub struct Report {
    result: String,
    fighters: Fighters,
}

#[derive(Serialize, ToSchema)]
pub struct Fighters {
    pub you: Fighter,
    pub opponent: Fighter,
}

#[derive(Serialize)]
pub struct Fighter {
    pokemon: PokemonDto,
    life: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    attack: Option<StaticString>,
}

#[derive(Serialize)]
pub struct PokemonDto {
    name: StaticString,
    life: u16,
}

#[derive(Serialize, ToSchema)]
pub struct AttackOption {
    attack: StaticString,
    method: StaticString,
    link: String,
}

impl From<BattleReport> for Battle {
    fn from(report: BattleReport) -> Self {
        let battle_id = report.id.to_string();
        let attacks = report.fighters.0.pokemon.attacks();
        let battle_root = format!("/battle/{}", battle_id);

        let is_over = report.clone().is_over();

        let attacks_options: Vec<AttackOption> = attacks.into_iter().map(|attack| AttackOption {
            attack: attack.name(),
            method: "post",
            link: format!("{}/attack/{}", battle_root, attack.name().to_ascii_lowercase()),
        }).collect();

        Battle {
            report: Report {
                result: report.result.to_string(),
                fighters: Fighters {
                    you: Fighter {
                        pokemon: PokemonDto {
                            name: report.fighters.0.pokemon.name(),
                            life: report.fighters.0.pokemon.life(),
                        },
                        life: report.fighters.0.life,
                        attack: report.attacks.map(|attack| attack.0.name()),
                    },
                    opponent: Fighter {
                        pokemon: PokemonDto {
                            name: report.fighters.1.pokemon.name(),
                            life: report.fighters.1.pokemon.life(),
                        },
                        life: report.fighters.1.life,
                        attack: report.attacks.map(|attack| attack.1.name()),
                    },
                },
            },
            attack_options: if is_over { vec![] } else { attacks_options },
        }
    }
}