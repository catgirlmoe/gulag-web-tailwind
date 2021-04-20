/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::schema::users::users;


joinable!(users -> stats_vn_std (id));
allow_tables_to_appear_in_same_query!(users, stats_vn_std);
table! {
  #[sql_name = "stats"]
  stats_vn_std (id) {
    id -> Integer,
    #[sql_name = "tscore_vn_std"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_vn_std"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_vn_std"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_vn_std"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_vn_std"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_vn_std"]
    acc -> Float,
    #[sql_name = "max_combo_vn_std"]
    max_combo -> Unsigned<Integer>,
  }
}

joinable!(users -> stats_vn_taiko (id));
allow_tables_to_appear_in_same_query!(users, stats_vn_taiko);
table! {
  #[sql_name = "stats"]
  stats_vn_taiko (id) {
    id -> Integer,
    #[sql_name = "tscore_vn_taiko"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_vn_taiko"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_vn_taiko"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_vn_taiko"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_vn_taiko"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_vn_taiko"]
    acc -> Float,
    #[sql_name = "max_combo_vn_taiko"]
    max_combo -> Unsigned<Integer>,
  }
}

joinable!(users -> stats_vn_catch (id));
allow_tables_to_appear_in_same_query!(users, stats_vn_catch);
table! {
  #[sql_name = "stats"]
  stats_vn_catch (id) {
    id -> Integer,
    #[sql_name = "tscore_vn_catch"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_vn_catch"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_vn_catch"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_vn_catch"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_vn_catch"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_vn_catch"]
    acc -> Float,
    #[sql_name = "max_combo_vn_catch"]
    max_combo -> Unsigned<Integer>,
  }
}

joinable!(users -> stats_vn_mania (id));
allow_tables_to_appear_in_same_query!(users, stats_vn_mania);
table! {
  #[sql_name = "stats"]
  stats_vn_mania (id) {
    id -> Integer,
    #[sql_name = "tscore_vn_mania"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_vn_mania"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_vn_mania"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_vn_mania"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_vn_mania"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_vn_mania"]
    acc -> Float,
    #[sql_name = "max_combo_vn_mania"]
    max_combo -> Unsigned<Integer>,
  }
}


joinable!(users -> stats_rx_std (id));
allow_tables_to_appear_in_same_query!(users, stats_rx_std);
table! {
  #[sql_name = "stats"]
  stats_rx_std (id) {
    id -> Integer,
    #[sql_name = "tscore_rx_std"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_rx_std"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_rx_std"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_rx_std"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_rx_std"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_rx_std"]
    acc -> Float,
    #[sql_name = "max_combo_rx_std"]
    max_combo -> Unsigned<Integer>,
  }
}

joinable!(users -> stats_rx_taiko (id));
allow_tables_to_appear_in_same_query!(users, stats_rx_taiko);
table! {
  #[sql_name = "stats"]
  stats_rx_taiko (id) {
    id -> Integer,
    #[sql_name = "tscore_rx_taiko"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_rx_taiko"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_rx_taiko"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_rx_taiko"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_rx_taiko"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_rx_taiko"]
    acc -> Float,
    #[sql_name = "max_combo_rx_taiko"]
    max_combo -> Unsigned<Integer>,
  }
}

joinable!(users -> stats_rx_catch (id));
allow_tables_to_appear_in_same_query!(users, stats_rx_catch);
table! {
  #[sql_name = "stats"]
  stats_rx_catch (id) {
    id -> Integer,
    #[sql_name = "tscore_rx_catch"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_rx_catch"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_rx_catch"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_rx_catch"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_rx_catch"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_rx_catch"]
    acc -> Float,
    #[sql_name = "max_combo_rx_catch"]
    max_combo -> Unsigned<Integer>,
  }
}


joinable!(users -> stats_ap_std (id));
allow_tables_to_appear_in_same_query!(users, stats_ap_std);
table! {
  #[sql_name = "stats"]
  stats_ap_std (id) {
    id -> Integer,
    #[sql_name = "tscore_ap_std"]
    tscore -> Unsigned<BigInt>,
    #[sql_name = "rscore_ap_std"]
    rscore -> Unsigned<BigInt>,
    #[sql_name = "pp_ap_std"]
    pp -> Unsigned<Integer>,
    #[sql_name = "plays_ap_std"]
    plays -> Unsigned<Integer>,
    #[sql_name = "playtime_ap_std"]
    playtime -> Unsigned<Integer>,
    #[sql_name = "acc_ap_std"]
    acc -> Float,
    #[sql_name = "max_combo_ap_std"]
    max_combo -> Unsigned<Integer>,
  }
}