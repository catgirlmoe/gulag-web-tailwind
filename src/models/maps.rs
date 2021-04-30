/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::schema::maps::maps;
use crate::db::conn;
use crate::qformat;
use crate::models::QueryFormat;

use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, result::Error};
use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Map {
  pub server: String,
  pub id: i32,
  pub set_id: i32,
  pub status: i32,
  pub md5: String,
  pub artist: String,
  pub title: String,
  pub version: String,
  pub creator: String,
  pub last_update: NaiveDateTime,
  pub total_length: i32,
  pub max_combo: i32,
  pub frozen: bool,
  pub plays: i32,
  pub passes: i32,
  pub mode: i8,
  pub bpm: f32,
  pub cs: f32,
  pub ar: f32,
  pub od: f32,
  pub hp: f32,
  pub diff: f32,
}

impl Map {
  pub fn maps(q: QueryFormat) -> Result<Vec<Self>, Error> {
    qformat!(maps::table, q, ("plays" => maps::plays)).load::<Self>(&conn())
  }

  pub fn from_id<'a>(id: i32) -> Result<Self, Error> {
    maps::table.find(id).first::<Self>(&conn())
  }

  pub fn from_md5<'a>(map_md5: String) -> Result<Self, Error> {
    maps::table.filter(maps::md5.eq(map_md5)).first::<Self>(&conn())
  }
}