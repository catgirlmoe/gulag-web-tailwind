/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

table! {
  maps (id) {
    server -> Text,
    id -> Integer,
    set_id -> Integer,
    status -> Integer,
    md5 -> Text,
    artist -> Text,
    title -> Text,
    version -> Text,
    creator -> Text,
    last_update -> Datetime,
    total_length -> Integer,
    max_combo -> Integer,
    frozen -> Bool,
    plays -> Integer,
    passes -> Integer,
    mode -> TinyInt,
    bpm -> Float,
    cs -> Float,
    ar -> Float,
    od -> Float,
    hp -> Float,
    diff -> Float,
  }
}