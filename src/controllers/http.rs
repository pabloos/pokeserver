

use std::net::Ipv4Addr;
use std::io::Result;
use actix_web::web::Data;
use actix_web::{HttpServer, App};
use actix_cors::Cors;

use crate::core::str::StaticString;

use super::handlers::DynPokeApp;
use super::routes::routes;

// from https://stackoverflow.com/questions/65645622/how-do-i-pass-a-trait-as-application-data-to-actix-web
pub async fn run_server(swagger: StaticString, spec: StaticString, app: DynPokeApp, port: u16) -> Result<()> {    
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(Data::new(app.clone()))
            .configure(routes(swagger, spec))
    }).bind((Ipv4Addr::UNSPECIFIED, port))?.run().await
}