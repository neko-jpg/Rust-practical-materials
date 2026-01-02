//! my_utils クレート
//!
//! このクレートは「Rust実践カリキュラム」全体で使用するユーティリティ関数群を提供します。
//! ここでは、ドキュメントコメントの書き方とテストの作法を学びます。

/// 2つの `i32` を加算します。
///
/// # Examples
///
/// ```
/// let result = my_utils::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 簡易的なロガーを初期化します。
/// 環境変数 `RUST_LOG` が設定されていない場合は `info` レベルで出力します。
pub fn setup_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
