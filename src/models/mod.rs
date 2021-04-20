/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

pub mod maps;
pub mod scores;
pub mod stats;
pub mod users;

// Do something with this later
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryFormat {
  pub sort: Option<String>,
  pub asc: Option<bool>,
  pub limit: Option<i64>,
  pub offset: Option<i64>,
  pub page: Option<i64>,
}

#[macro_export] // a little shitcode is better than a lot
macro_rules! qformat {
  ($table:expr, $opts:expr, ($($column_name:expr => $column:expr),*)) => {
    if $opts.sort != None {
      match $opts.sort.unwrap().as_str() {
        $(
          $column_name => if ($opts.asc != None && $opts.asc.unwrap()) {
            $table.into_boxed().order($column.asc())
          } else {
            $table.into_boxed().order($column.desc())
          },
        )*
        _ => $table.into_boxed()
      }
    } else {
      $table.into_boxed()
    }.limit(if($opts.limit != None) {std::cmp::min($opts.limit.unwrap(), 50)} else {50})
    .offset(if($opts.offset != None) {$opts.offset.unwrap()} else {0} + 
            if($opts.page != None) {$opts.page.unwrap()} else {0} * 
            if($opts.limit != None) {$opts.limit.unwrap()} else {50})
  }
}