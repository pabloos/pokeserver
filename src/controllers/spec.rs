use utoipa::OpenApi;

use super::dto::{Battle, PokemonRes, Report};
use super::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_pokemon,
        handlers::list_pokemons,
        handlers::create_battle,
        handlers::attack
    ),
    components(schemas(Battle, Report, PokemonRes),)
)]
pub struct ApiDoc;
