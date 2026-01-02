use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Chat Server running on 127.0.0.1:8080");

    // TODO 1: ブロードキャストチャネルを作成する
    // (tx, _rx) = broadcast::channel(10); 
    // キャパシティは10メッセージ分
    let (tx, _rx) = broadcast::channel(10);

    loop {
        // TODO 2: 接続を受け入れる (listener.accept().await)
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("New connection: {}", addr);

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
                // select! マクロで「ソケットからの受信」と「チャネルからの受信」を同時に待つ
                tokio::select! {
                    // ソケットからメッセージが来た場合（クライアント -> サーバー）
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break; // 接続切れ
                        }
                        // TODO 4: 受け取ったメッセージを全クライアントにブロードキャスト送信する
                        // tx.send(...)
                        print!("Received from {}: {}", addr, line);
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    
                    // 他のクライアントからのメッセージがチャネルに来た場合（サーバー -> クライアント）
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        // 自分以外のメッセージなら書き込む
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
