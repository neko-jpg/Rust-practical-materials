use tracing::{info, instrument, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // ログ設定: 標準出力にJSON形式で出力
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    info!("Starting application...");

    process_request("req-123").await;
    process_request("req-999").await;

    info!("Application finished.");
}

// #[instrument] マクロを使うと、関数の引数が自動的にログのフィールドに含まれます
#[instrument]
async fn process_request(request_id: &str) {
    info!("Processing request");
    
    // 擬似的な重い処理
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    if request_id == "req-999" {
        warn!(user_id = 5, "Suspicious request detected");
    }

    sub_task(42).await;
}

#[instrument]
async fn sub_task(value: i32) {
    info!(result = value * 2, "Calculated result");
}
