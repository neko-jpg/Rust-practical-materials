use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Level 2.5: トレイトによる抽象化
// リクエストを処理する機能を持ったものを「Handler」として定義します。
// これにより、具体的な処理内容を入れ替えることが可能になります。
pub trait Handler {
    fn handle(&self, stream: std::net::TcpStream);
}

// Level 2.8: マルチスレッド同期 (ThreadPool)
// スレッドを管理し、タスクを各スレッドに分配する仕組みです。
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// スレッドプールに送信するタスクの型定義
// Box<dyn FnBox + Send + 'static> のような動的ディスパッチを利用します。
// ここでは簡略化のために、FnOnceクロージャをBox化したものをJobと呼びます。
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 新しいThreadPoolを作成します。
    ///
    /// size が 0 以下の場合はパニックします。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // Receiverは複数のスレッドで共有する必要があるため、
        // Arc (Atomic Reference Counting) と Mutex (Mutual Exclusion) で包みます。
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // TODO: Workerを作成し、ベクタに追加する
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// クロージャを受け取り、それを空いているスレッドで実行します。
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // チャネルを通じてJobを送信
        if let Some(ref sender) = self.sender {
            sender.send(job).expect("Failed to send job to worker");
        }
    }
}

// Dropトレイトを実装して、ThreadPoolがスコープを抜けたときに
// スレッドを適切に終了させる（Graceful Shutdown）
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // senderをドロップしてチャネルを閉じる
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // スレッドの終了を待機する
            if let Some(thread) = worker.thread.take() {
                // スレッドがパニックしていても、ここでは終了を待つだけなのでエラーは無視またはログ出力
                if let Err(_) = thread.join() {
                    eprintln!("Worker {} thread panicked", worker.id);
                }
            }
        }
    }
}

// 実際にスレッドを保持し、Jobを待機する労働者
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // スレッドを生成し、ループでJobを待ち受ける
        let thread = thread::spawn(move || loop {
            // ロックを取得してメッセージを受信
            // recv() はチャネルが閉じられるまでブロックする
            // lock() が失敗した場合は PoisonError なので、一般的にはパニックさせるか回復を試みる
            // ここでは簡易的に expect を使用
            let message = receiver.lock().expect("Mutex is poisoned").recv();

            match message {
                Ok(job) => {
                    // println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    // Senderがドロップされるとここに来る
                    // println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
