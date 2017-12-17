use infrastructure::repository_impl;
use infrastructure::Conn;

pub struct NanachiRepository {
    pub db_file: String // データベースファイル
}


impl NanachiRepository {

    /// new
    pub fn new(db_file: &str) -> NanachiRepository {
        NanachiRepository {
            db_file: db_file.to_owned()
        }
    }

    /// 単語の追加
    pub fn add_word(&self, conn: &mut Conn, name: &str, value: &str) -> Result<(), String> {
        repository_impl::add_pairs(conn,"word", "name", "text", name, value)
    }

    /// 最初の文節の追加
    pub fn add_pf(&self, conn: &mut Conn, value: &str) -> Result<(), String> {
        repository_impl::add_one_text(conn, "pf", value)
    }

    /// 間の文節の追加
    pub fn add_pa(&self, conn: &mut Conn, value: &str) -> Result<(), String> {
        repository_impl::add_one_text(conn, "pa", value)
    }

    /// 最後の文節の追加
    pub fn add_pe(&self, conn: &mut Conn, value: &str) -> Result<(), String> {
        repository_impl::add_one_text(conn, "pe", value)
    }

    /// 最初の文節をランダムに取得
    pub fn get_pf_random(&self, conn: &mut Conn) -> Result<String, String> {
        repository_impl::get_one_random_text(conn, "pf")
    }

    /// 間の文節をランダムに取得
    pub fn get_pa_random(&self, conn: &mut Conn) -> Result<String, String> {
        repository_impl::get_one_random_text(conn, "pa")
    }

    /// 最後の文節をランダムに取得
    pub fn get_pe_random(&self, conn: &mut Conn) -> Result<String, String> {
        repository_impl::get_one_random_text(conn, "pe")
    }

    /// 単語をランダムに取得
    pub fn get_word_random(&self, conn: &mut Conn, word: &str) -> Result<String, String> {
        repository_impl::get_one_random_text_by_word(conn, "word", &word)
    }

    /// テーブル作成
    pub fn create_table(&self, conn: &mut Conn) -> Result<(), String> {
        repository_impl::create_table(conn, &self.db_file)
    }

}