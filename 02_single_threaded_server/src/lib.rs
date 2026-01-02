use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Level 2.5: トレイトによる抽象化
pub trait Handler {
    fn handle(&self, stream: std::net::TcpStream);
}

// Level 2.8: マルチスレッド同期 (ThreadPool)
pub struct ThreadPool {
    // TODO: スレッド(Worker)と送信側チャネル(Sender)を保持するフィールドを定義
    // workers: Vec<Worker>,
    // sender: Option<mpsc::Sender<Job>>,
}

// スレッドプールに送信するタスクの型定義
#[allow(dead_code)]
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 新しいThreadPoolを作成します。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // TODO: チャネルを作成し、Workerを生成する
        // let (sender, receiver) = mpsc::channel();
        // let receiver = Arc::new(Mutex::new(receiver));

        ThreadPool {
            // workers,
            // sender: Some(sender),
        }
    }

    /// クロージャを受け取り、それを空いているスレッドで実行します。
    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Jobを作成し、チャネル経由で送信する
    }
}

// Dropトレイトを実装して、ThreadPoolがスコープを抜けたときに
// スレッドを適切に終了させる（Graceful Shutdown）
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: 全てのWorkerを終了させる
    }
}

// 実際にスレッドを保持し、Jobを待機する労働者
// (この構造体はプライベートでOK)
#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

#[allow(dead_code)]
impl Worker {
    #[allow(unused_variables)]
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: thread::spawn でスレッドを生成し、receiverからJobを受け取るループを作る

        let thread = thread::spawn(move || {
            // loop { ... }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
