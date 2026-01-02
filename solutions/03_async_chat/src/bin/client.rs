use tokio::io::stdin;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO 1: サーバーに接続する
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
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
                match result {
                    Ok(0) => {
                         println!("Server closed connection");
                         break;
                    }
                    Ok(_) => {
                        print!("Server: {}", server_line);
                    }
                    Err(e) => {
                        eprintln!("Error reading from server: {}", e);
                        break;
                    }
                }
            }

            // ユーザーがキーボード入力したとき
            result = stdin_reader.read_line(&mut stdin_line) => {
                match result {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        // TODO 3: 入力された内容をサーバーに送信する
                        if let Err(e) = writer.write_all(stdin_line.as_bytes()).await {
                             eprintln!("Failed to write to server: {}", e);
                             break;
                        }
                        stdin_line.clear();
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdin: {}", e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
