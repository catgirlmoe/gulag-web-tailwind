/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

table! {
  users (id) {
    id -> Integer,
    name -> Text,
    #[sql_name = "priv"]
    perms -> Integer,
    country -> Text,
    creation_time -> Integer,
    latest_activity -> Integer,
    clan_id -> Integer,
    clan_priv -> TinyInt,
  }
}