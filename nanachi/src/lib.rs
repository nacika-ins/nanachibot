pub mod application;
pub mod domain;
pub mod infrastructure;

#[cfg(test)]
mod tests {

    use application::service::nanachi::NanachiApplicationService;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /// 投稿テスト
    #[test]
    fn post() {

        let nanachi = NanachiApplicationService::new("db.sqlite3");
        let result = nanachi.word_input("nanachi ナナチ");
        println!("{:?}", result);


    }
}
