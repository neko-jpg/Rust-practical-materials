use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Chat Server running on 127.0.0.1:8080");

    // TODO 1: ブロードキャストチャネルを作成する
    // let (tx, _rx) = broadcast::channel(10);

    loop {
        // TODO 2: 接続を受け入れる (listener.accept().await)
        // let (mut socket, addr) = ...

        // ダミー（無限ループ回避のため）
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Waiting for connections...");
        continue;

        /*
        // クライアントごとに送信チャンネルのクローンを作成
        let tx = tx.clone();
        // 受信チャンネルを作成
        let mut rx = tx.subscribe();

        // TODO 3: タスクをspawnして非同期に処理する
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                // TODO 4: select! マクロでソケット受信とチャネル受信を同時に待つ
                tokio::select! {
                    // ...
                }
            }
        });
        */
    }
}
