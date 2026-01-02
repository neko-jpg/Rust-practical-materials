use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// use single_threaded_server::ThreadPool; // Level 2.8で有効化

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");

    // TODO: ThreadPoolを初期化する (Level 2.8)
    // let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TODO: ここでリクエストを処理する
        // シングルスレッドの場合: handle_connection(stream);
        // マルチスレッドの場合 (Level 2.8): pool.execute(|| handle_connection(stream));

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // TODO: リクエストを解析してレスポンスを返す
    // 1. "GET / HTTP/1.1" かどうか確認
    // 2. "GET /sleep HTTP/1.1" かどうか確認 (マルチスレッド動作確認用)
    // 3. ファイル (hello.html / 404.html) を読み込んで返す

    // ダミーレスポンス
    let response = "HTTP/1.1 200 OK\r\n\r\nHello";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
