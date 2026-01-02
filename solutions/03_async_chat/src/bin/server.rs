use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Chat Server running on 127.0.0.1:8080");

    // TODO 1: ブロードキャストチャネルを作成する
    // (tx, _rx) = broadcast::channel(10);
    // キャパシティは10メッセージ分
    let (tx, _rx) = broadcast::channel(10);

    loop {
        // TODO 2: 接続を受け入れる (listener.accept().await)
        // 接続エラーがあってもループを回し続ける
        let (mut socket, addr) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };
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
                         match result {
                            Ok(0) => break, // Connection closed
                            Ok(_) => {
                                // TODO 4: 受け取ったメッセージを全クライアントにブロードキャスト送信する
                                // tx.send(...)
                                print!("Received from {}: {}", addr, line);
                                // メッセージ送信失敗（受信者がいないなど）は無視して継続
                                let _ = tx.send((line.clone(), addr));
                                line.clear();
                            }
                            Err(e) => {
                                eprintln!("Error reading from socket: {}", e);
                                break;
                            }
                        }
                    }

                    // 他のクライアントからのメッセージがチャネルに来た場合（サーバー -> クライアント）
                    result = rx.recv() => {
                        match result {
                            Ok((msg, other_addr)) => {
                                // 自分以外のメッセージなら書き込む
                                if addr != other_addr {
                                    if let Err(e) = writer.write_all(msg.as_bytes()).await {
                                        eprintln!("Failed to write to socket: {}", e);
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Broadcast receive error: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
    }
}
