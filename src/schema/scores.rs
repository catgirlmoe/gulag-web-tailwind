/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::schema::{maps::maps, users::users};


allow_tables_to_appear_in_same_query!(scores_vn, users);
allow_tables_to_appear_in_same_query!(scores_vn, maps);
table! {
  scores_vn (id) {
    id -> Unsigned<BigInt>,
    map_md5 -> Text,
    score -> Integer,
    pp -> Float,
    acc -> Float,
    max_combo -> Integer,
    mods -> Integer,
    n300 -> Integer,
    n100 -> Integer,
    n50 -> Integer,
    nmiss -> Integer,
    ngeki -> Integer,
    nkatu -> Integer,
    grade -> Text,
    status -> TinyInt,
    mode -> TinyInt,
    play_time -> Datetime,
    time_elapsed -> Integer,
    #[sql_name = "userid"]
    user_id -> Integer,
    perfect -> Bool,
  }
}

allow_tables_to_appear_in_same_query!(scores_rx, users);
allow_tables_to_appear_in_same_query!(scores_rx, maps);
table! {
  scores_rx (id) {
    id -> Unsigned<BigInt>,
    map_md5 -> Text,
    score -> Integer,
    pp -> Float,
    acc -> Float,
    max_combo -> Integer,
    mods -> Integer,
    n300 -> Integer,
    n100 -> Integer,
    n50 -> Integer,
    nmiss -> Integer,
    ngeki -> Integer,
    nkatu -> Integer,
    grade -> Text,
    status -> TinyInt,
    mode -> TinyInt,
    play_time -> Datetime,
    time_elapsed -> Integer,
    #[sql_name = "userid"]
    user_id -> Integer,
    perfect -> Bool,
  }
}

allow_tables_to_appear_in_same_query!(scores_ap, users);
allow_tables_to_appear_in_same_query!(scores_ap, maps);
table! {
  scores_ap (id) {
    id -> Unsigned<BigInt>,
    map_md5 -> Text,
    score -> Integer,
    pp -> Float,
    acc -> Float,
    max_combo -> Integer,
    mods -> Integer,
    n300 -> Integer,
    n100 -> Integer,
    n50 -> Integer,
    nmiss -> Integer,
    ngeki -> Integer,
    nkatu -> Integer,
    grade -> Text,
    status -> TinyInt,
    mode -> TinyInt,
    play_time -> Datetime,
    time_elapsed -> Integer,
    #[sql_name = "userid"]
    user_id -> Integer,
    perfect -> Bool,
  }
}