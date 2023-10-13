// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.




use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    // 创建一个 Arc 来包装队列，以便在多个线程之间共享
    let qc = Arc::new(q);

    // 克隆 Arc 以便在两个线程中使用
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    // 克隆 Sender 以便在两个线程中使用
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // 在一个新线程中发送队列的第一半数据
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("发送: {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 在另一个新线程中发送队列的第二半数据
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("发送: {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // 创建一个通道，用于线程之间的通信
    let (tx, rx) = mpsc::channel();
    
    // 创建队列
    let queue = Queue::new();
    
    // 获取队列的长度
    let queue_length = queue.length;

    // 启动发送线程
    send_tx(queue, tx);

    // 接收线程接收数据并计数
    let mut total_received: u32 = 0;
    for received in rx {
        println!("收到: {}", received);
        total_received += 1;
    }

    // 打印接收到的总数，并进行断言
    println!("总共接收到的数字: {}", total_received);
    assert_eq!(total_received, queue_length);
}
