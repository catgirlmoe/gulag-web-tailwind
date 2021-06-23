/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::{QueryFormat, maps::Map, scores::Score};
use crate::error::SeverError;

use actix_web::{get, HttpResponse, Scope, web, web::{Path, Query}};

#[get("")]
async fn maps(q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Map::maps(q.into_inner())?))
}

#[get("/{map_md5}")]
async fn get_map(Path(map_md5): Path<String>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Map::from_md5(map_md5)?))
}

#[get("/{map_md5}/scores/vn/std")]
async fn get_scores_vn_std(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_vn_std(map_md5, q.into_inner())?))
}
#[get("/{map_md5}/scores/vn/taiko")]
async fn get_scores_vn_taiko(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_vn_taiko(map_md5, q.into_inner())?))
}
#[get("/{map_md5}/scores/vn/catch")]
async fn get_scores_vn_catch(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_vn_catch(map_md5, q.into_inner())?))
}
#[get("/{map_md5}/scores/vn/mania")]
async fn get_scores_vn_mania(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_vn_mania(map_md5, q.into_inner())?))
}

#[get("/{map_md5}/scores/rx/std")]
async fn get_scores_rx_std(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_rx_std(map_md5, q.into_inner())?))
}
#[get("/{map_md5}/scores/rx/taiko")]
async fn get_scores_rx_taiko(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_rx_taiko(map_md5, q.into_inner())?))
}
#[get("/{map_md5}/scores/rx/catch")]
async fn get_scores_rx_catch(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_rx_catch(map_md5, q.into_inner())?))
}

#[get("/{map_md5}/scores/ap/std")]
async fn get_scores_ap_std(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, SeverError> {
  Ok(HttpResponse::Ok().json(Score::map_scores_ap_std(map_md5, q.into_inner())?))
}

pub fn scope() -> Scope {
  web::scope("/maps")
    .service(maps)
    .service(get_map)
    .service(get_scores_vn_std)
    .service(get_scores_vn_taiko)
    .service(get_scores_vn_catch)
    .service(get_scores_vn_mania)
    .service(get_scores_rx_std)
    .service(get_scores_rx_taiko)
    .service(get_scores_rx_catch)
    .service(get_scores_ap_std)
} 