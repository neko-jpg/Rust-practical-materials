use axum::{
    routing::get,
    Router,
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::net::SocketAddr;
use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handler;
mod model;

#[tokio::main]
async fn main() {
    // ロギングの初期化
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 環境変数の読み込み (.env)
    dotenvy::dotenv().ok();
    // DATABASE_URLがなければデフォルト値を使用（ファイルベースのSQLite）
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://products.db?mode=rwc".to_string());

    // DB接続プールを作成する
    // ConnectOptionsを使って、ファイルが存在しない場合に作成するように設定
    let options = SqliteConnectOptions::from_str(&db_url)
        .expect("Invalid database URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("can't connect to database");

    // 初期化SQLを実行（テーブル作成）
    // 本番ではマイグレーションツールを使うべきですが、学習用として簡易的に実装
    let schema = std::fs::read_to_string("schema.sql").expect("Failed to read schema.sql");
    sqlx::query(&schema)
        .execute(&pool)
        .await
        .expect("Failed to initialize database");
    tracing::info!("Database initialized");

    // アプリケーションのルーティング設定
    // 状態（DBプール）を共有するために .with_state(pool) を使う
    let app = Router::new()
        .route(
            "/products",
            get(handler::list_products).post(handler::create_product),
        )
        .route("/products/:id", get(handler::get_product))
        .with_state(pool);

    // サーバー起動
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
