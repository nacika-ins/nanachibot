extern crate sqlite3;

pub mod repository_impl;

use self::sqlite3::{DatabaseConnection, SqliteResult, SqliteError, StatementUpdate, ResultRowAccess, Query};
use self::sqlite3::access::open;
use self::sqlite3::access::flags::OpenFlags;

/// インフラストラクチャのラッパーコネクション
pub type Conn = DatabaseConnection;

/// コネクションの取得
pub fn get_conn(db_file: &str) -> Result<Conn, String> {
    let conn_result: Result<Conn, _> =
        sqlite3::access::open(db_file,
                              Some(sqlite3::access::flags::OpenFlags::default())).map_err(|err| format!("err: {}", err.to_string()));
    conn_result
}