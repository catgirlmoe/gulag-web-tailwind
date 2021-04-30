/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::{QueryFormat, maps::Map, scores::Score};
use crate::error::NekosuError;

use actix_web::{get, HttpResponse, Scope, web, web::{Path, Query}};

#[get("")]
async fn maps(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Map::maps(q.into_inner())?))
}

#[get("/{map_md5}")]
async fn get_map(Path(map_md5): Path<String>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Map::from_md5(map_md5)?))
}

#[get("/{map_md5}/vn_scores")]
async fn get_vn_scores(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Score::map_vn_scores(map_md5, q.into_inner())?))
}

#[get("/{map_md5}/rx_scores")]
async fn get_rx_scores(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Score::map_rx_scores(map_md5, q.into_inner())?))
}

#[get("/{map_md5}/ap_scores")]
async fn get_ap_scores(Path(map_md5): Path<String>, q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Score::map_ap_scores(map_md5, q.into_inner())?))
}

pub fn scope() -> Scope {
  web::scope("/maps")
    .service(maps)
    .service(get_map)
    .service(get_vn_scores)
    .service(get_rx_scores)
    .service(get_ap_scores)
} 