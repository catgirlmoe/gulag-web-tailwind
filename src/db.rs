/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use diesel::MysqlConnection;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
use lazy_static::lazy_static;
use std::env;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

lazy_static! {
    static ref POOL: MysqlPool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
}

pub fn conn() -> DbConnection {
    POOL.get().expect("Oh no my system crashed, now i lost my deta")
}