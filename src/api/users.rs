/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::users::User;
use crate::error::SeverError;

use actix_web::{get, HttpResponse, web, Scope};


#[get("/{user_id}")]
async fn get(web::Path(user_id): web::Path<i32>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(User::find_by_id(user_id)?))
}

pub fn scope() -> Scope {
  web::scope("/users")
    .service(get)
} 