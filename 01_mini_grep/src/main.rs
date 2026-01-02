use std::env;
use std::error::Error;
use std::process;

fn main() {
    // 1. コマンドライン引数を取得します
    let args: Vec<String> = env::args().collect();

    // Config構造体を使って引数を解析する
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("引数の解析中に問題が発生しました: {}", err);
        process::exit(1);
    });

    println!("検索文字列: {}", config.query);
    println!("対象ファイル: {}", config.filename);

    // run関数を呼び出し、エラーがあれば表示して終了する
    if let Err(e) = run(config) {
        eprintln!("アプリケーションエラー: {}", e);
        process::exit(1);
    }
}

// アプリケーションのロジック
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO: ファイルの内容を読み込む
    // ヒント: fs::read_to_string(...)
    let contents = "";

    // 検索を行い、結果を表示する
    for line in search(&config.query, contents) {
        println!("{}", line);
    }

    Ok(())
}

// 検索ロジック
// query と contents の参照を受け取り、contents のライフタイムに紐付いたスライスを返します
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {


    // TODO: 行ごとに反復処理し、queryを含む行をresultsに追加する
    // ヒント: contents.lines() を使う

    Vec::new()
}

// 設定を保持する構造体
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // 引数のベクタを受け取り、Configインスタンスまたはエラーメッセージを返す
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引数が足りません。使用法: <検索文字列> <ファイル名>");
        }

        // TODO: args[1] と args[2] を取得して Config を作成する
        let query = String::new(); // 修正してください
        let filename = String::new(); // 修正してください

        Ok(Config { query, filename })
    }
}
