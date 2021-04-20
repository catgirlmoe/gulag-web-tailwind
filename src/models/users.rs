/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::schema::users::users;
use crate::db::conn;

use diesel::{QueryDsl, RunQueryDsl};
use serde::Serialize;


#[derive(Queryable, Serialize)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub perms: i32,
  pub country: String,
  pub creation_time: i32,
  pub latest_activity: i32,
  pub clan_id: i32,
  pub clan_priv: i8,
}

impl User {
  pub fn find_by_id<'a>(id: i32) -> Result<User, diesel::result::Error> {
    return users::table.find(id).first::<User>(&conn());
  }
}