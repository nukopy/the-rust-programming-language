use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    println!("===== sec02 =====");
    message_passing_hello_world();
    message_passing_mpsc();
}

fn message_passing_hello_world() {
    println!("===== message_passing_hello_world =====");
    // init transmitter, receiver（転送機と受信機の初期化）
    // mpsc: multiple producer, single consumer
    // 簡潔に言えば、Rust の標準ライブラリがチャンネルを実装している方法は、1 つのチャンネルが値を生成する複数の送信側と、その値を消費するたった1つの受信側を持つことができるということを意味する
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("sending: {}", val);

        // tx.send メソッドは引数の所有権を奪い、値がムーブされると、受信側が所有権を得る。
        // これにより、送信後に誤って再度値を使用するのを防いでくれる。所有権システムが、万事問題ないことを確認してくれる。
        tx.send(val).unwrap();
        // println!("sent: {}", val); tx.send に所有権が奪われるためこれは実行できない
    });

    // rx.recv メソッドを実行すると、メインスレッドの実行をブロックし、値がチャンネルを流れてくるまで待機する。
    // 一旦値が送信されたら、recv はそれを Result<T, E> に含んで返す。チャンネルの送信側が閉じたら、recv はエラーを返し、もう値は来ないと通知する。
    let received = rx.recv().unwrap();
    println!("received: {}", received);
}

fn message_passing_mpsc() {
    println!("===== message_passing_mpsc =====");
    let (tx, rx) = mpsc::channel::<String>();
    let tx1 = mpsc::Sender::clone(&tx);

    // spawned thread 1
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in values {
            println!("[tx] Sending: {}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // spawned thread 2
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in values {
            println!("[tx1] Sending: {}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 2 つのスレッドからのメッセージを rx で受信する際の到着順序は保証されない
    // rx が閉じられる（= すべての tx がドロップされる）まで無限に待つように、for ループを使っている。そして、受け取ったメッセージを出力する。
    for received in rx {
        println!("Received: {}", received);
    }
}
