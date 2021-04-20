/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_web::{web, Scope};

mod home;
mod lbs;


struct Config<'a> {//TODO: replace this with actual config
  name: &'a str,
  domain: &'a str
}

struct Session {
  authenticated: bool,
  user_id: i32
}

const CONFIG: Config = Config {
  name: "nekosu!",
  domain: "osu.catgirl.moe"
};

const SESSION: Session = Session { //Temporary placeholder stuff
  authenticated: false,
  user_id: 1
};

pub fn scope() -> Scope {
  web::scope("")
    .service(home::home)
    .service(lbs::lbs)
} 