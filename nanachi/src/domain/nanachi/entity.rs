use domain::nanachi;
use infrastructure;
use infrastructure::Conn;

extern crate rand;

extern crate regex;
use self::regex::Regex;

use std::str::FromStr;

extern crate time;

/// Nanachi
pub struct Nanachi {
    db_file: String,
    repository: nanachi::repository::NanachiRepository
}

const RANDOM_RATE: u8 = 3; // 中間文節が出る割合
const LOOP_MAX_COUNT: i64 = 1000; // ループ最大カウント


/// Nanachi 実装
impl Nanachi {
    pub fn new(db_file: &str) -> Nanachi {
        Nanachi { db_file: db_file.to_owned(), repository: nanachi::repository::NanachiRepository::new(db_file) }
    }

    /// 単語をlineから登録
    pub fn word_input_from_line(&self, line: &str) -> Result<(), String> {
        let mut connection = infrastructure::get_conn(&self.db_file)?;
        let re = Regex::new(r"^([^\s]+)\s(.*)+$").map_err(|err| format!("err: {}", err.to_string()))?;
        for cap in re.captures_iter(line) {
            if cap.at(1).is_some() && cap.at(2).is_some() {
                let name = cap.at(1).ok_or("error name")?;
                let value = cap.at(2).ok_or("error value")?;
                self.word_input(&mut connection, name, value);
            } else {
                return Err("err".to_owned())
            }
        }
        Ok(())
    }

    /// 単語の登録
    pub fn word_input(&self, conn: &mut Conn, name: &str, value: &str) -> Result<(), String> {
        self.repository.add_word(conn, name, value)
    }

    /// 文章学習
    /// | で区切る
    pub fn sentense_input(&self, line: &str) -> Result<(), String> {
        let mut conn = infrastructure::get_conn(&self.db_file)?;
        let words: Vec<&str> = line.split("|").collect();

        match words.len() {
            1 => {
                let _ = self.repository.add_pe(&mut conn, words.last().expect("last"))?;
            }
            2 => {
                let _ = self.repository.add_pf(&mut conn, words.first().expect("first"))?;
                let _ = self.repository.add_pe(&mut conn, words.last().expect("last"))?;
            }
            0 => {  }
            _ => {
                let _ = self.repository.add_pf(&mut conn, words.first().expect("first"))?;
                let _ = self.repository.add_pe(&mut conn, words.last().expect("last"))?;
                let mut words = words;
                words.pop();
                words.remove(0);
                for i in words {
                    let _ = self.repository.add_pa(&mut conn, i.clone())?;
                }
            }
        }
        Ok(())
    }

    /// 発言
    pub fn post(&self) -> Result<String, String> {

        let mut conn = infrastructure::get_conn(&self.db_file)?;
        let mut message = "".to_owned();
        let mut count = 0;

        // 短文判定
        let rnd = rand::random::<u8>();
        match rnd {
            _ if rnd % RANDOM_RATE == 0 => {
                let last_text = self.repository.get_pe_random(&mut conn)?;
                message.push_str(&last_text);
            }
            _ => {
                loop {

                    message = "".to_owned();

                    // 始文節の追加
                    let first_text = self.repository.get_pf_random(&mut conn)?;
                    message.push_str(&first_text);

                    // 中文節の追加
                    let mut previous_words: Vec<String> = vec![];
                    loop {
                        let middle_text = self.repository.get_pa_random(&mut conn)?;
                        if (!previous_words.contains(&middle_text)) {
                            message.push_str(&middle_text);
                            previous_words.push(middle_text.clone());
                        }

                        let rnd = rand::random::<u8>();
                        if rnd % RANDOM_RATE == 0 {
                            break;
                        }
                    }

                    // 終文節の追加
                    let last_text = self.repository.get_pe_random(&mut conn)?;
                    message.push_str(&last_text);

                    // ループカウントオーバー
                    if count == LOOP_MAX_COUNT {
                        break;
                    }
                    count += 1;

                    // 条件付きメッセージ
                    match self.condition_message(&message) {
                        Ok(v) => {
                            message = v;
                            break;
                        }
                        Err(e) => {}
                    };

                }
            }
        }


        // 単語の置換
        message = self.replace_words(&mut conn, &message)?;

        Ok(message)
    }


    /// コメントに対する返信
    pub fn post_comment(&self, comment: &str) -> Result<String, String> {

        let mut conn = infrastructure::get_conn(&self.db_file)?;
        let mut message = "".to_owned();
        let mut count = 0;

        // 短文判定
        let rnd = rand::random::<u8>();
        match rnd {
            _ if rnd % RANDOM_RATE == 0 => {
                let last_text = self.repository.get_pe_random(&mut conn)?;
                message.push_str(&last_text);
            }
            _ => {
                loop {

                    message = "".to_owned();

                    // 始文節の追加
                    let first_text = self.repository.get_pf_random(&mut conn)?;
                    message.push_str(&first_text);

                    // 中文節の追加
                    let mut previous_words: Vec<String> = vec![];
                    loop {
                        let middle_text = self.repository.get_pa_random(&mut conn)?;
                        if (!previous_words.contains(&middle_text)) {
                            message.push_str(&middle_text);
                            previous_words.push(middle_text.clone());
                        }

                        let rnd = rand::random::<u8>();
                        if rnd % RANDOM_RATE == 0 {
                            break;
                        }
                    }

                    // 終文節の追加
                    let last_text = self.repository.get_pe_random(&mut conn)?;
                    message.push_str(&last_text);

                    // ループカウントオーバー
                    if count == LOOP_MAX_COUNT {
                        break;
                    }
                    count += 1;

                    // 条件付きメッセージ
                    match self.condition_message_with_comment(&message, comment) {
                        Ok(v) => {
                            message = v;
                            break;
                        }
                        Err(e) => {}
                    };

                }
            }
        }


        // 単語の置換
        message = self.replace_words(&mut conn, &message)?;

        Ok(message)
    }

    /// 最初の文節をランダムに取得
    pub fn first_text(&self) -> Result<String, String> {
        let mut conn = infrastructure::get_conn(&self.db_file)?;
        let text = self.repository.get_pf_random(&mut conn)?;
        Ok(text)
    }

    /// 最後の文節をランダムに取得
    pub fn last_text(&self) -> Result<String, String> {
        let mut conn = infrastructure::get_conn(&self.db_file)?;
        let text = self.repository.get_pe_random(&mut conn)?;
        Ok(text)
    }

    /// 間の文節をランダムに取得
    pub fn middle_text(&self) -> Result<String, String> {
        let mut conn = infrastructure::get_conn(&self.db_file)?;
        let text = self.repository.get_pa_random(&mut conn)?;
        Ok(text)
    }

    /// 単語の置換を行う
    fn replace_words(&self, conn: &mut Conn, message: &str) -> Result<String, String> {
        let mut message: String = message.to_owned();
        let re = Regex::new(r"^.*%([^%]+)%.*$").map_err(|err| format!("err: {}", err.to_string()))?;
        loop {
            let mut new_message: String = message.clone().to_owned();
            let mut is_break = false;
            let mut is_find = false;
            for cap in re.captures_iter(&*message.clone()) {
                if cap.at(1).is_some() {
                    is_find = true;
                    let word = cap.at(1).unwrap();
                    if word == "" {
                        is_break = true;
                        break;
                    }
                    let choice_word = self.repository.get_word_random(conn, word).unwrap_or("".to_owned());
                    new_message = message.replace(&format!("%{}%", word), &choice_word).clone();
                    break;
                } else {
                    is_break = true;
                    break;
                }
            }
            if !is_find {
                break;
            }
            if is_break {
                break;
            }
            message = new_message.to_owned();
            if message == "" {
                break;
            }
        }
        Ok(message.to_owned())
    }

    /// テーブルの作成
    pub fn create_table(&self) -> Result<(), String> {
        let mut conn = infrastructure::get_conn(&self.db_file)?;
        self.repository.create_table(&mut conn)
    }

    /// コメントによる条件付きメッセージ
    /// %c:ヒットするキーワード%
    fn condition_message_with_comment(&self, message: &str, comment: &str) -> Result<String, String> {
        let mut message = message.to_owned();
        let re = Regex::new(r"^.*%c:([^%]+)%.*$").unwrap();
        for cap in re.captures_iter(&*message.clone()) {
            if cap.at(1).is_some() {
                let match_word = cap.at(1).unwrap();
                if comment.find(match_word).is_some() {
                    message = message.replace(&format!("%c:{}%", match_word), "");
                    return Ok(message);
                } else {
                }
            }
        }
        Err("not found".to_owned())
    }

    /// 条件付きメッセージ
    /// %t:0-5%  0時〜5時の間しか発言できない投稿
    /// %d:1224-1225% 指定した日時しか投稿できない投稿
    fn condition_message(&self, message: &str) -> Result<String, String> {
        let mut message = message.to_owned();
        let mut is_retry = false;

        // 時間制限
        let re = Regex::new(r"^.*%t:(\d+)-(\d+)%.*$").unwrap();
        for cap in re.captures_iter(&*message.clone()) {
            if cap.at(1).is_some() && cap.at(2).is_some() {
                is_retry = true;
                let now = time::now();
                let now_hour: i32 = now.tm_hour;
                let start_hour_str = cap.at(1).unwrap();
                let start_hour = i32::from_str(start_hour_str).unwrap();
                let end_hour_str = cap.at(2).unwrap();
                let end_hour = i32::from_str(end_hour_str).unwrap();
                if start_hour <= now_hour && end_hour >= now_hour {
                    message = message.replace(&format!("%t:{}-{}%", start_hour_str, end_hour_str),
                                              "");
                    return Ok(message);
                } else {
                }
            }
        }

        // 日付制限
        // %d:1224-1225%
        let re = Regex::new(r"^.*%d:(\d{2})(\d{2})-(\d{2})(\d{2})%.*$").unwrap();
        for cap in re.captures_iter(&*message.clone()) {
            if cap.at(1).is_some() && cap.at(2).is_some() {
                is_retry = true;
                let now = time::now();
                let now_month: i32 = now.tm_mon + 1;
                let now_day: i32 = now.tm_mday;

                let start_month_str = cap.at(1).unwrap();
                let start_month = i32::from_str(start_month_str).unwrap();

                let start_day_str = cap.at(2).unwrap();
                let start_day = i32::from_str(start_day_str).unwrap();

                let end_month_str = cap.at(3).unwrap();
                let end_month = i32::from_str(end_month_str).unwrap();

                let end_day_str = cap.at(4).unwrap();
                let end_day = i32::from_str(end_day_str).unwrap();

                if (start_month <= now_month && end_month >= now_month) &&
                    (start_day <= now_day && end_day >= now_day) {
                    message = message.replace(&format!("%d:{}{}-{}{}%",
                                                       start_month_str,
                                                       start_day_str,
                                                       end_month_str,
                                                       end_day_str),
                                              "");
                    return Ok(message);
                }
            }
        }
        if is_retry {
            Err("not found".to_owned())
        } else {
            Ok(message)
        }


    }


}
