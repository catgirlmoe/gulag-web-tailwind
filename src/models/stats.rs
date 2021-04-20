/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::schema::users::users;
use crate::models::QueryFormat;
use crate::schema::stats::*;
use crate::db::conn;
use crate::qformat;

use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, result::Error};
use serde::Serialize;


#[derive(Queryable, Serialize)]
pub struct Stats {
  pub id: i32,
  pub name: String,
  pub country: String,
  pub tscore: u64,
  pub rscore: u64,
  pub pp: u32,
  pub plays: u32,
  pub playtime: u32,
  pub acc: f32,
  pub max_combo: u32,
}

macro_rules! stats {
  ($schema:ident, $q:expr) => {
    qformat!($schema::table, $q, (
      "pp" => $schema::pp,
      "tscore" => $schema::tscore,
      "rscore" => $schema::rscore,
      "plays" => $schema::plays,
      "playtime" => $schema::playtime,
      "acc" => $schema::acc,
      "max_combo" => $schema::max_combo
    )).inner_join(users::table)
      .select((
        $schema::id,
        users::name,
        users::country,
        $schema::tscore,
        $schema::rscore,
        $schema::pp,
        $schema::plays,
        $schema::playtime,
        $schema::acc,
        $schema::max_combo
      ))
  }
}

impl Stats {
  pub fn get_vn_std(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_vn_std, q).filter(stats_vn_std::pp.gt(0)).load::<Stats>(&conn())
  }
  pub fn get_vn_taiko(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_vn_taiko, q).filter(stats_vn_taiko::pp.gt(0)).load::<Stats>(&conn())
  }
  pub fn get_vn_catch(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_vn_catch, q).filter(stats_vn_catch::pp.gt(0)).load::<Stats>(&conn())
  }
  pub fn get_vn_mania(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_vn_mania, q).filter(stats_vn_mania::pp.gt(0)).load::<Stats>(&conn())
  }

  pub fn get_rx_std(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_rx_std, q).filter(stats_rx_std::pp.gt(0)).load::<Stats>(&conn())
  }
  pub fn get_rx_taiko(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_rx_taiko, q).filter(stats_rx_taiko::pp.gt(0)).load::<Stats>(&conn())
  }
  pub fn get_rx_catch(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_rx_catch, q).filter(stats_rx_catch::pp.gt(0)).load::<Stats>(&conn())
  }

  pub fn get_ap_std(q: QueryFormat) -> Result<Vec<Self>, Error> {
    stats!(stats_ap_std, q).filter(stats_ap_std::pp.gt(0)).load::<Stats>(&conn())
  }
}