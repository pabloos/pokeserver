
use std::sync::Arc;

use actix_web::{post, get, web, Responder};

use crate::{controllers::dto::{Battle, PokemonRes}, core::ports::app::PokeApp};

pub type DynPokeApp = Arc<dyn PokeApp + Send + Sync + 'static>;

#[get("/greet")]
async fn greet() -> impl Responder {
    "Welcome to the pokedex. Try to search somethig... lets GET /pokedex/pikachu"
}

#[utoipa::path(
    path = "/pokedex",
    responses(
        (status = 200, description = "Pokemon found succesfully", body = Vec<PokemonRes>)
    ),
)]
#[get("")]
async fn list_pokemons(app: web::Data<DynPokeApp>) -> impl Responder {
    let list: Vec<PokemonRes> = app.list_pokemons()
                                   .into_iter().map(PokemonRes::from)
                                   .collect();

    web::Json(list)
}

#[utoipa::path(
    path = "/pokedex/{name}",
    responses(
        (status = 200, description = "Pokemon found succesfully", body = Pokemon),
        (status = NOT_FOUND, description = "Pokemon not found")
    ),
    params(
        ("name" = String, Path, description = "Name of the pokemon to look up"),
    )
)]
#[get("/{name}")]
pub async fn get_pokemon(app: web::Data<DynPokeApp>, path: web::Path<String>) -> impl Responder {    
    let pokemon_name = path.into_inner();
    let pokemon = app.get_pokemon(pokemon_name).unwrap();

    web::Json(PokemonRes::from(pokemon))
}

#[utoipa::path(
    path = "/pokedex/{pokemon}/fight",
    responses(
        (status = 200, description = "Pokemon found succesfully", body = Battle)
    ),
)]
#[post("/{pokemon}/fight")]
async fn create_battle(path: web::Path<String>, app: web::Data<DynPokeApp>) -> impl Responder {
    let pokemon_name = path.into_inner();
    
    let report = app.create_battle(pokemon_name);

    web::Json(Battle::from(report))
}

#[utoipa::path(
    path = "/battle/{battle_id}/attack/{attack_name}",
    params(
        ("battle_id" = String, Path, description = "battle id to continue"),
        ("attack_name" = String, Path, description = "attack name to continue"),
    )
)]
#[post("/{battle_id}/attack/{attack_name}")]
async fn attack(path: web::Path<(String, String)>, app: web::Data<DynPokeApp>) -> impl Responder {
    let (battle_id, attack_name) = path.into_inner();

    let report = app.continue_battle(battle_id, attack_name);

    web::Json(Battle::from(report))
}
