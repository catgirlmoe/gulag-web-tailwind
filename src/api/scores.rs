/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::scores::Score;
use crate::error::SeverError;

use actix_web::{get, HttpResponse, web, Scope};


#[get("/{score_id}")]
async fn get(web::Path(score_id): web::Path<u64>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::from_id(score_id)?))
}

pub fn scope() -> Scope {
  web::scope("/scores")
    .service(get)
} 