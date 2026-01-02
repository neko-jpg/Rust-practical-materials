use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::model::{CreateProduct, Product};

// TODO 1: 商品一覧を取得するハンドラ
pub async fn list_products(
    State(_pool): State<SqlitePool>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    // ヒント: sqlx::query_as
    todo!("Implement list_products")
}

// TODO 2: 商品を作成するハンドラ
pub async fn create_product(
    State(_pool): State<SqlitePool>,
    Json(_payload): Json<CreateProduct>,
) -> Result<Json<Product>, (StatusCode, String)> {
    // ヒント: INSERT INTO ...
    todo!("Implement create_product")
}

// TODO 3: 特定の商品を取得するハンドラ
pub async fn get_product(
    State(_pool): State<SqlitePool>,
    Path(_id): Path<i64>,
) -> Result<Json<Product>, (StatusCode, String)> {
    // ヒント: WHERE id = ?
    todo!("Implement get_product")
}
