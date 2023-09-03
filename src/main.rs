use std::sync::Arc;

use webserver::adapters::{app, services::storage};
use webserver::config::{PORT, SPEC_ROUTE, SWAGGER_ROUTE};
use webserver::controllers::http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = storage::new_pokemon_storage();
    let app = Arc::new(app::new_pokeapp(storage));

    http::run_server(SWAGGER_ROUTE, SPEC_ROUTE, app, PORT).await
}
