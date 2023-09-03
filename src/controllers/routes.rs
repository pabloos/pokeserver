use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::core::str::StaticString;

use super::handlers::{attack, create_battle, get_pokemon, greet, list_pokemons};
use super::spec::ApiDoc;

pub fn routes( swagger: StaticString, spec: StaticString) -> impl FnOnce(&mut web::ServiceConfig) + 'static {
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/pokedex")
                .service(greet)
                .service(list_pokemons)
                .service(get_pokemon)
                .service(create_battle),
        )
        .service(web::scope("/battle").service(attack))
        .service(SwaggerUi::new(swagger).url(spec, ApiDoc::openapi()));
    }
}