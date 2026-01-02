use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // TODO 1: リスナーをローカルホストの7878ポートにバインドする
    // let listener = TcpListener::bind(...).unwrap();
    println!("Server running on 127.0.0.1:7878");

    // TODO 2: 着信する接続(stream)をループで処理する
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream);
    // }
}

fn handle_connection(mut stream: TcpStream) {
    // バッファを用意してリクエストを読み込む
    let mut buffer = [0; 1024];
    
    // TODO 3: streamからbufferにデータを読み込む
    // stream.read(&mut buffer).unwrap();

    // デバッグ用にリクエストを表示（バイト列を文字列に変換）
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // シンプルなレスポンスの作成
    // GET / HTTP/1.1 なら 200 OK、それ以外なら 404 Not Found を返すようにしてみましょう

    let get = b"GET / HTTP/1.1\r\n";

    // TODO 4: リクエストの内容（buffer）が `get` で始まっているか確認する
    if buffer.starts_with(get) {
        // TODO 5: 正常なレスポンスを返す
        // ファイルを読み込んでボディにするのが一般的ですが、まずは固定文字列でOK
        // "HTTP/1.1 200 OK\r\n\r\nHello, Rust!"
        
        let status_line = "HTTP/1.1 200 OK";
        let contents = "<h1>Hello from Rust!</h1>"; // ファイル読み込みに変えてみよう: fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{}
Content-Length: {}

{}",
            status_line, length, contents
        );

        // TODO 6: streamにレスポンスを書き込み、flushする
        // stream.write(response.as_bytes()).unwrap();
        // stream.flush().unwrap();
    } else {
        // TODO 7: 404レスポンスを返す
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = "<h1>404 - Not Found</h1>";
        let length = contents.len();

        let response = format!(
            "{}
Content-Length: {}

{}",
            status_line, length, contents
        );

        // stream.write(response.as_bytes()).unwrap();
        // stream.flush().unwrap();
    }
}
