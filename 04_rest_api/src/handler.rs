use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::model::{CreateProduct, Product};

// TODO 1: 商品一覧を取得するハンドラ
// State<SqlitePool> でDBプールを受け取る
pub async fn list_products(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    
    // sqlx::query_as! マクロを使うとコンパイル時にSQLチェックができるが、
    // 環境構築の簡便さのため query_as 関数を使用する
    let products = sqlx::query_as::<_, Product>("SELECT id, name, price FROM products")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(products))
}

// TODO 2: 商品を作成するハンドラ
pub async fn create_product(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProduct>,
) -> Result<Json<Product>, (StatusCode, String)> {
    
    let id = sqlx::query("INSERT INTO products (name, price) VALUES (?, ?)")
        .bind(&payload.name)
        .bind(payload.price)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .last_insert_rowid();

    let product = Product {
        id,
        name: payload.name,
        price: payload.price,
    };

    Ok(Json(product))
}

// TODO 3: 特定の商品を取得するハンドラ
pub async fn get_product(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, (StatusCode, String)> {
    
    // optional: データがない場合は404を返す処理が必要
    let product = sqlx::query_as::<_, Product>("SELECT id, name, price FROM products WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Product not found".to_string()))?;

    Ok(Json(product))
}
