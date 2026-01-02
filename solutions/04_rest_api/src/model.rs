use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// DBのテーブル定義に対応する構造体
// FromRow: クエリ結果を構造体にマッピングする
#[derive(Debug, Serialize, FromRow)]
pub struct Product {
    pub id: i64, // SQLiteのINTEGERはi64に対応
    pub name: String,
    pub price: i64,
}

// APIリクエスト（入力）用の構造体
#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: i64,
}
