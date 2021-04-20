/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_web::{web, Scope};

mod maps;
mod scores;
mod stats;
mod users;

pub fn scope() -> Scope {
  web::scope("/api")
    .service(maps::scope())
    .service(scores::scope())
    .service(stats::scope())
    .service(users::scope())
}