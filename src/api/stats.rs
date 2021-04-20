/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::{QueryFormat, stats::Stats};
use crate::error::NekosuError;

use actix_web::{get, HttpResponse, web, web::Query, Scope};


#[get("/vn/std")]
async fn get_vn_std(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_vn_std(q.into_inner())?))
}
#[get("/vn/taiko")]
async fn get_vn_taiko(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_vn_taiko(q.into_inner())?))
}
#[get("/vn/catch")]
async fn get_vn_catch(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_vn_catch(q.into_inner())?))
}
#[get("/vn/mania")]
async fn get_vn_mania(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_vn_mania(q.into_inner())?))
}

#[get("/rx/std")]
async fn get_rx_std(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_rx_std(q.into_inner())?))
}
#[get("/rx/taiko")]
async fn get_rx_taiko(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_rx_taiko(q.into_inner())?))
}
#[get("/rx/catch")]
async fn get_rx_catch(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_rx_catch(q.into_inner())?))
}

#[get("/ap/std")]
async fn get_ap_std(q: Query<QueryFormat>) -> Result<HttpResponse, NekosuError> {
  Ok(HttpResponse::Ok().json(Stats::get_ap_std(q.into_inner())?))
}

pub fn scope() -> Scope {
  web::scope("/stats")
    .service(get_vn_std)
    .service(get_vn_taiko)
    .service(get_vn_catch)
    .service(get_vn_mania)
    .service(get_rx_std)
    .service(get_rx_taiko)
    .service(get_rx_catch)
    .service(get_ap_std)
} 