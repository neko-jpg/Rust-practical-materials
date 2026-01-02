use tokio::io::stdin;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // TODO 1: サーバーに接続する
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connected to server!");

    // ストリームを読み込み用と書き込み用に分割
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    // 標準入力の準備
    let mut stdin_reader = BufReader::new(stdin());
    let mut stdin_line = String::new();

    loop {
        let mut server_line = String::new();

        // TODO 2: サーバーからの受信と、標準入力からの読み込みを同時に待機する (select!)
        tokio::select! {
            // サーバーからデータが来たとき
            result = reader.read_line(&mut server_line) => {
                let bytes_read = result.unwrap();
                if bytes_read == 0 {
                    println!("Server closed connection");
                    break;
                }
                print!("Server: {}", server_line);
            }

            // ユーザーがキーボード入力したとき
            result = stdin_reader.read_line(&mut stdin_line) => {
                let _ = result.unwrap();
                // TODO 3: 入力された内容をサーバーに送信する
                writer.write_all(stdin_line.as_bytes()).await.unwrap();
                stdin_line.clear();
            }
        }
    }
}
