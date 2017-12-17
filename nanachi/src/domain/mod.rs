pub mod nanachi;

#[cfg(test)]
mod tests {

    use domain::nanachi::entity::Nanachi;

    /// 投稿テスト
    #[test]
    fn post() {

        let nanachi = Nanachi::new("db.sqlite3");
        let _ = nanachi.create_table();

        let _ = nanachi.word_input_from_line("nanachi ナナチ");
        let _ = nanachi.word_input_from_line("tabemono 奈落鍋");
        let _ = nanachi.word_input_from_line("tabemono ナキカバネの肉");

        let _ = nanachi.sentense_input("今日は|ご飯を|たくさん|食べた");
        // let _ = nanachi.sentense_input("昨日は|ご飯を|たくさん|食べた");
        // let _ = nanachi.sentense_input("ナナチは|かわいいですね");
        // let _ = nanachi.sentense_input("ナナチは|かわいいですね");
        // let _ = nanachi.sentense_input("美味しいですよね");
        // let _ = nanachi.sentense_input("奈落鍋が|食べたい|んなぁ〜");
        // let _ = nanachi.sentense_input("んなぁ〜");
        let _ = nanachi.sentense_input("%c:メリクリ%メリークリスマスんなぁ〜");
        let _ = nanachi.sentense_input("%t:20-23%ねむいんぁ〜");
        let _ = nanachi.sentense_input("%t:20-23%ねむいんぁ〜");
        let _ = nanachi.sentense_input("%t:06-09%おはよんぁ〜");
        let _ = nanachi.sentense_input("%t:06-09%おはよんぁ〜");
        let _ = nanachi.sentense_input("%tabemono%は|おいしいんなぁ〜");
        let _ = nanachi.first_text();

        let result = nanachi.post();
        println!("？「こんにちは」");
        println!("ナナチ: 「{}」", result.expect("get"));

        let result = nanachi.post();
        println!("？「こんにちは」");
        println!("ナナチ: 「{}」", result.expect("get"));

        let result = nanachi.post();
        println!("？「こんにちは」");
        println!("ナナチ: 「{}」", result.expect("get"));

        let result = nanachi.post();
        println!("？「こんにちは」");
        println!("ナナチ: 「{}」", result.expect("get"));

        let result = nanachi.post();
        println!("？「こんにちは」");
        println!("ナナチ: 「{}」", result.expect("get"));

        let result = nanachi.post();
        println!("？「こんにちは」");
        println!("ナナチ: 「{}」", result.expect("get"));

        let result = nanachi.post_comment("メリクリ");
        println!("？「メリクリ」");
        println!("ナナチ: 「{}」", result.expect("get"));





    }
}