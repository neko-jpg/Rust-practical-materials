use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // 1. コマンドライン引数を取得します
    let args: Vec<String> = env::args().collect();

    // 引数の数をチェック (プログラム名 + 引数2つ = 3つ必要)
    if args.len() < 3 {
        eprintln!("使用法: {} <検索文字列> <ファイル名>", args[0]);
        process::exit(1);
    }

    // TODO 1: Config構造体を使って引数を解析する処理を呼び出す
    // let config = Config::new(&args).unwrap_or_else(|err| { ... });
    // ここでは簡易的に直接変数に代入します（あとで構造体にリファクタリングしてみましょう）
    let query = &args[1];
    let filename = &args[2];

    println!("検索文字列: {}", query);
    println!("対象ファイル: {}", filename);

    // TODO 2: run関数を呼び出し、エラーがあれば表示して終了する
    if let Err(e) = run(query, filename) {
        eprintln!("アプリケーションエラー: {}", e);
        process::exit(1);
    }
}

// アプリケーションのロジック
// TODO 3: 引数の型を適切に設定し、Result<(), Box<dyn Error>> を返すようにする
fn run(query: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    // TODO 4: ファイルの内容を読み込む
    // ヒント: fs::read_to_string(filename)?
    let contents = ""; // ここを実装

    // TODO 5: 検索を行い、結果を表示する
    // search関数を呼び出して、結果をループで表示する
    for line in search(query, contents) {
        println!("{}", line);
    }

    Ok(())
}

// 検索ロジック
// TODO 6: ライフタイム注釈を追加する
// query と contents の参照を受け取り、contents のスライスを含むベクタを返すため、
// ライフタイムの指定が必要です。
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // TODO 7: 行ごとに反復処理し、queryを含む行をresultsに追加する
    // ヒント: contents.lines()

    results
}

// 設定を保持する構造体（発展課題）
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // TODO 8 (発展): 引数のベクタを受け取り、Configインスタンスまたはエラーメッセージを返す
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引数が足りません");
        }
        // clone() を使ってStringを生成してもOKです（学習段階なので）
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
