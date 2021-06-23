/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::{QueryFormat, users::User, scores::Score};
use crate::error::SeverError;

use actix_web::{get, HttpResponse, web, web::{Path, Query}, Scope};


#[get("/{user_id}")]
async fn get_user(Path(user_id): web::Path<i32>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(User::find_by_id(user_id)?))
}

#[get("/{user_id}/scores/vn/std")]
async fn get_scores_vn_std(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_vn_std(user_id, q.into_inner())?))
}
#[get("/{user_id}/scores/vn/taiko")]
async fn get_scores_vn_taiko(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_vn_taiko(user_id, q.into_inner())?))
}
#[get("/{user_id}/scores/vn/catch")]
async fn get_scores_vn_catch(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_vn_catch(user_id, q.into_inner())?))
}
#[get("/{user_id}/scores/vn/mania")]
async fn get_scores_vn_mania(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_vn_mania(user_id, q.into_inner())?))
}

#[get("/{user_id}/scores/rx/std")]
async fn get_scores_rx_std(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_rx_std(user_id, q.into_inner())?))
}
#[get("/{user_id}/scores/rx/taiko")]
async fn get_scores_rx_taiko(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_rx_taiko(user_id, q.into_inner())?))
}
#[get("/{user_id}/scores/rx/catch")]
async fn get_scores_rx_catch(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_rx_catch(user_id, q.into_inner())?))
}

#[get("/{user_id}/scores/ap/std")]
async fn get_scores_ap_std(Path(user_id): Path<i32>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::user_scores_ap_std(user_id, q.into_inner())?))
}

pub fn scope() -> Scope {
  web::scope("/users")
    .service(get_user)
    .service(get_scores_vn_std)
    .service(get_scores_vn_taiko)
    .service(get_scores_vn_catch)
    .service(get_scores_vn_mania)
    .service(get_scores_rx_std)
    .service(get_scores_rx_taiko)
    .service(get_scores_rx_catch)
    .service(get_scores_ap_std)
} 