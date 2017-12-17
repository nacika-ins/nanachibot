use super::sqlite3::{DatabaseConnection, SqliteResult, SqliteError, StatementUpdate, ResultRowAccess, Query};
use super::sqlite3::access::open;
use super::sqlite3::access::flags::OpenFlags;
use infrastructure::Conn;

/// 指定したテーブルにペアとなるレコードを追加
pub fn add_pairs( connection: &mut Conn, table: &str, table_key: &str, table_value: &str, key: &str, value: &str ) -> Result<(), String> {
    let sql = format!("insert into {}({}, {}) values(?,?);", table, table_key, table_value);
    let mut statement = connection.prepare(&sql).map_err(|err| format!("err: {}", err.to_string()))?;
    let _ = statement.bind_text(1, key).map_err(|err| format!("err: {}", err.to_string()))?;
    let _ = statement.bind_text(2, value).map_err(|err| format!("err: {}", err.to_string()))?;
    statement.execute().step().map_err(|err| format!("err: {}", err.to_string()))?;
    Ok(())
}

/// ひとつのテキストを追加
pub fn add_one_text(connection: &mut Conn, table: &str, value: &str) -> Result<(), String> {
    let sql = format!("insert into {}(text) values(?);", table);
    let mut statement = connection.prepare(&sql).map_err(|err| format!("err: {}", err.to_string()))?;
    let _ = statement.bind_text(1, value).map_err(|err| format!("err: {}", err.to_string()))?;
    statement.execute().step().map_err(|err| format!("err: {}", err.to_string()))?;
    Ok(())
}


/// ひとつのテキストをランダムに取得
pub fn get_one_random_text(connection: &mut Conn, table: &str) -> Result<String, String> {
    let mut text = "".to_owned();
    loop {
        let mut stmt = connection.prepare(&format!("SELECT text FROM {} ORDER BY RANDOM() LIMIT 1", table)).map_err(|err| format!("err: {}", err.to_string()))?;
        let mut rows = stmt.query(&[], |row| {
            Ok(row.get("text"))
        }).map_err(|err| format!("err: {}", err.to_string()))?;
        let mut text_result = rows.next().ok_or("not found text 1")?;
        let text: String = text_result.map_err(|err| format!("err: {}", err.to_string()))?;
        if text.find("%userid%").is_none() {
            return Ok(text);
        }
    }
    Ok(text)
}

/// ひとつのテキストをランダムに取得
pub fn get_one_random_text_by_word(connection: &mut Conn, table: &str, word: &str) -> Result<String, String> {
    let mut text = "".to_owned();
    loop {
        let mut stmt = connection.prepare(&format!("SELECT text FROM {} WHERE name = ? ORDER BY RANDOM() LIMIT 1", table)).map_err(|err| format!("err: {}", err.to_string()))?;
        let _ = stmt.bind_text(1, word).map_err(|err| format!("err: {}", err.to_string()))?;
        let mut rows = stmt.query(&[], |row| {
            Ok(row.get("text"))
        }).map_err(|err| format!("err: {}", err.to_string()))?;
        let mut text_result = rows.next().ok_or(format!("not found text 2 word => {}", word))?;
        let text: String = text_result.map_err(|err| format!("err: {}", err.to_string()))?;
        if text.find("%userid%").is_none() {
            return Ok(text);
        }
    }
    Ok(text)
}

/// テーブルの作成
pub fn create_table(conn: &mut Conn, db_file: &str) -> Result<(), String> {

    // pf
    let r = conn.exec(&format!(r#"
            CREATE TABLE {} (text TEXT);
        "#, "pf"));
    conn.changes();

    // pa
    let r = conn.exec(&format!(r#"
            CREATE TABLE {} (text TEXT);
        "#, "pa"));
    conn.changes();

    // pe
    let r = conn.exec(&format!(r#"
            CREATE TABLE {} (text TEXT);
        "#, "pe"));
    conn.changes();

    // word
    let r = conn.exec(&format!(r#"
            CREATE TABLE {} (name verptr(255), text TEXT);
            create index nameindex on {}(name);
        "#, "word", "word"));
    conn.changes();

    Ok(())
}