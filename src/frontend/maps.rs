/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_web::{get, HttpResponse, HttpRequest};
use actix_http::http::header::ContentType;
use askama::Template;
use super::*;


#[derive(Template)]
#[template(path = "pages/maps.html")]
struct LeadeboardPage<'a> {
  config: &'a Config<'a>,
  session: &'a Session,
  path: &'a str,
}

#[get("/maps")]
async fn maps(req: HttpRequest) -> HttpResponse {
  let body = LeadeboardPage {
    config: &CONFIG,
    session: &SESSION,
    path: req.path()
  }
  .render()
  .unwrap();
  HttpResponse::Ok().set(ContentType(mime::TEXT_HTML)).body(body)
}