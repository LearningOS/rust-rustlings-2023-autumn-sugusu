// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// 定义一个结构体 Queue，模拟一个队列
struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        // 创建并返回一个新的队列实例
        Queue {
            length: 10,                      // 设置队列长度为 10
            first_half: vec![1, 2, 3, 4, 5], // 初始化前半部分
            second_half: vec![6, 7, 8, 9, 10], // 初始化后半部分
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    // 克隆 Arc 共享指针，以便在闭包中使用
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    // 启动一个线程来发送前半部分的数据
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("发送: {:?}", val);
            let tx = tx1.lock().unwrap(); // 获取锁
            tx.send(*val).unwrap();      // 发送数据到通道
            thread::sleep(Duration::from_secs(1)); // 暂停 1 秒
        }
    });

    // 启动另一个线程来发送后半部分的数据
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("发送: {:?}", val);
            let tx = tx2.lock().unwrap(); // 获取锁
            tx.send(*val).unwrap();      // 发送数据到通道
            thread::sleep(Duration::from_secs(1)); // 暂停 1 秒
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();  // 创建一个多生产者单消费者通道
    let queue = Queue::new();       // 创建一个新的队列实例
    let queue_length = queue.length; // 获取队列的长度

    // 创建一个带锁的共享 Sender，以便多个线程可以安全地使用它
    let tx = Arc::new(Mutex::new(tx));

    // 启动发送数据的线程
    send_tx(queue, Arc::clone(&tx));

    let mut total_received: u32 = 0;
    for received in rx {
        println!("接收到: {}", received);
        total_received += 1;
    }

    println!("总共接收到的数字: {}", total_received);
    assert_eq!(total_received, queue_length);  // 断言总接收数等于队列长度
}



