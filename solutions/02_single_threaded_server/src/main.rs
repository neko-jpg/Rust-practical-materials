use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

// lib.rs で定義した ThreadPool と Handler を利用
use single_threaded_server::ThreadPool;
// use single_threaded_server::Handler; // Traitを使う場合はここでuseする

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Server running on http://127.0.0.1:7878 (Multi-threaded)");

    // スレッドプールを作成（4つのスレッド）
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Level 2.8: ThreadPool を使って並列処理
                pool.execute(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    // readは読み込んだバイト数を返す。0なら接続終了など。
    // ここでは単純化のため戻り値をチェックするが、unwrapはしない。
    let _ = stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // マルチスレッドの効果を確認するための重い処理
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // ファイル読み込みのエラーハンドリングを少し丁寧に
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => String::from("<h1>File not found on server</h1>"),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
