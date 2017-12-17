extern crate sqlite3;
extern crate regex;

use self::sqlite3::{DatabaseConnection, SqliteResult, SqliteError, StatementUpdate, ResultRowAccess,
              Query};
use self::sqlite3::access::open;
use self::sqlite3::access::flags::OpenFlags;
use self::regex::Regex;


use domain;
use infrastructure;
use infrastructure::Conn;

/// Nanachi
pub struct NanachiApplicationService {
    pub nanachi: domain::nanachi::entity::Nanachi,
    db_file: String, // データベースファイル
}

/// Nanachiの実装
impl NanachiApplicationService {
    pub fn new(db_file: &str) -> NanachiApplicationService {
        let nanachi = NanachiApplicationService {
            nanachi: domain::nanachi::entity::Nanachi::new(db_file),
            db_file: db_file.to_owned()
        };
        nanachi.nanachi.create_table();
        nanachi
    }


    // 単語学習
    pub fn word_input(&self, line: &str) -> Result<(), String> {
        self.nanachi.word_input_from_line(line)
    }

    // 文章学習
    pub fn sentense_input(&self, line: &str) -> Result<(), String> {
        self.nanachi.sentense_input(line)
    }

    /// 発言
    pub fn post(&self) -> Result<String, String> {
        self.nanachi.post()
    }

    /// コメント返答
    pub fn post_comment(&self, comment: &str) -> Result<String, String> {
        self.nanachi.post_comment(comment)
    }
}