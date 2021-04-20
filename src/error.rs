/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_http::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{Error as DieselError};
use std::{convert::From, fmt::Debug};
use derive_more::Display; //TODO: Ditch single use dependency


#[derive(Debug, Display)]
pub enum NekosuError {
  NotFound,
  DieselError(String)
}

impl ResponseError for NekosuError {
  fn error_response(&self) -> HttpResponse {
    match self {
      NekosuError::NotFound => HttpResponse::BadRequest().status(StatusCode::NOT_FOUND).finish(),
      _ => HttpResponse::BadRequest().status(StatusCode::INTERNAL_SERVER_ERROR).finish()
    }
  }
}

impl From<DieselError> for NekosuError {
  fn from(err: DieselError) -> NekosuError {
    match err {
      DieselError::NotFound => NekosuError::NotFound,
      _ => NekosuError::DieselError(err.to_string())
    }
  }
}