/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::schema::scores::*;
use crate::db::conn;
use crate::qformat;
use crate::models::QueryFormat;

use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, result::Error};
use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Score {
  pub id: u64,
  pub map_md5: String,
  pub score: i32,
  pub pp: f32,
  pub acc: f32,
  pub max_combo: i32,
  pub mods: i32,
  pub n300: i32,
  pub n100: i32,
  pub n50: i32,
  pub nmiss: i32,
  pub ngeki: i32,
  pub nkatu: i32,
  pub grade: String,
  pub status: i8,
  pub mode: i8,
  pub play_time: NaiveDateTime,
  pub time_elapsed: i32,
  pub user_id: i32,
  pub perfect: bool,
}



impl Score {
  pub fn from_id(id: u64) -> Result<Score, Error> {
    if id < 3074457345618258602 {
      scores_vn::table.find(id).first::<Score>(&conn())
    } else if id < 6148914691236517204 {
      scores_rx::table.find(id).first::<Score>(&conn())
    } else {
      scores_ap::table.find(id).first::<Score>(&conn())
    }
  }

  pub fn map_scores_vn_std(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(0)).filter(scores_vn::map_md5.eq(map_md5)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }
  pub fn map_scores_vn_taiko(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(1)).filter(scores_vn::map_md5.eq(map_md5)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }
  pub fn map_scores_vn_catch(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(2)).filter(scores_vn::map_md5.eq(map_md5)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }
  pub fn map_scores_vn_mania(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(3)).filter(scores_vn::map_md5.eq(map_md5)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }

  pub fn map_scores_rx_std(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_rx::table.filter(scores_rx::mode.eq(0)).filter(scores_rx::map_md5.eq(map_md5)), q, ("pp" => scores_rx::pp, "acc" => scores_rx::acc)).load::<Score>(&conn())
  }
  pub fn map_scores_rx_taiko(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_rx::table.filter(scores_rx::mode.eq(1)).filter(scores_rx::map_md5.eq(map_md5)), q, ("pp" => scores_rx::pp, "acc" => scores_rx::acc)).load::<Score>(&conn())
  }
  pub fn map_scores_rx_catch(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_rx::table.filter(scores_rx::mode.eq(2)).filter(scores_rx::map_md5.eq(map_md5)), q, ("pp" => scores_rx::pp, "acc" => scores_rx::acc)).load::<Score>(&conn())
  }

  pub fn map_scores_ap_std(map_md5: String, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_ap::table.filter(scores_ap::mode.eq(0)).filter(scores_ap::map_md5.eq(map_md5)), q, ("pp" => scores_ap::pp, "acc" => scores_ap::acc)).load::<Score>(&conn())
  }


  pub fn user_scores_vn_std(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(0)).filter(scores_vn::user_id.eq(user_id)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }
  pub fn user_scores_vn_taiko(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(1)).filter(scores_vn::user_id.eq(user_id)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }
  pub fn user_scores_vn_catch(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(2)).filter(scores_vn::user_id.eq(user_id)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }
  pub fn user_scores_vn_mania(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_vn::table.filter(scores_vn::mode.eq(3)).filter(scores_vn::user_id.eq(user_id)), q, ("pp" => scores_vn::pp, "acc" => scores_vn::acc)).load::<Score>(&conn())
  }

  pub fn user_scores_rx_std(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_rx::table.filter(scores_rx::mode.eq(0)).filter(scores_rx::user_id.eq(user_id)), q, ("pp" => scores_rx::pp, "acc" => scores_rx::acc)).load::<Score>(&conn())
  }
  pub fn user_scores_rx_taiko(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_rx::table.filter(scores_rx::mode.eq(1)).filter(scores_rx::user_id.eq(user_id)), q, ("pp" => scores_rx::pp, "acc" => scores_rx::acc)).load::<Score>(&conn())
  }
  pub fn user_scores_rx_catch(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_rx::table.filter(scores_rx::mode.eq(2)).filter(scores_rx::user_id.eq(user_id)), q, ("pp" => scores_rx::pp, "acc" => scores_rx::acc)).load::<Score>(&conn())
  }

  pub fn user_scores_ap_std(user_id: i32, q: QueryFormat) -> Result<Vec<Score>, Error> {
    qformat!(scores_ap::table.filter(scores_ap::mode.eq(0)).filter(scores_ap::user_id.eq(user_id)), q, ("pp" => scores_ap::pp, "acc" => scores_ap::acc)).load::<Score>(&conn())
  }
}