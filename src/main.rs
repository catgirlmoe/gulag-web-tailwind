/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

#[macro_use] 
extern crate diesel;
extern crate actix_web;
extern crate serde_json;
extern crate mime;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use std::{io, env};

mod api;
mod frontend;

pub mod models;
pub mod schema;
pub mod db;
pub mod error;

#[actix_web::main]
async fn main() -> io::Result<()> {
  env_logger::init();
  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .service(Files::new("/assets", "./assets"))
      .service(api::scope())
      .service(frontend::scope())
  })
  .bind(format!("0.0.0.0:{}", env::var("PORT").unwrap()))?
  .run()
  .await
}