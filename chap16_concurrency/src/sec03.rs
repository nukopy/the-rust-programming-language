// use std::rc::Rc;
use rand;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// #[allow(dead_code)]
pub fn main() {
    println!("===== sec03 =====");
    shared_state_concurrency_hello_world();
    shared_state_concurrency_multi_thread();
}

// メモリを共有することでやりとりする（⇔ やりとりすることでメモリを拠有しろ）
fn shared_state_concurrency_hello_world() {
    println!("===== shared_state_concurrency_hello_world =====");

    let mutex = Mutex::<i32>::new(5);
    // let lock_before_num = mutex.lock().unwrap();

    {
        let mut num = mutex.lock().unwrap();
        *num = 6;
    }

    println!("mutex = {:?}", mutex);
}

// 以下のコードは借用チェックでコンパイルエラーになる
// counterの所有権を複数のスレッドに移すことはできない
/*
fn shared_state_concurrency_multi_thread() {
    println!("===== shared_state_concurrency_multi_thread =====");

    let counter_mutex = Mutex::<i32>::new(5);
    let mut handles = vec![];


    // 10 個のスレッドを生成して、それぞれのスレッドで counter_mutex をインクリメントする
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter_mutex.lock().unwrap();
            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter_mutex = {:?}", counter_mutex);
}
*/

// 以下のコードはコンパイルエラーになる
/*
残念ながら、Rc<T>はスレッド間で共有するには安全ではないのです。
Rc<T>が参照カウントを管理する際、 cloneが呼び出されるたびにカウントを追加し、クローンがドロップされるたびにカウントを差し引きます。
しかし、並行基本型を使用してカウントの変更が別のスレッドに妨害されないことを確認していないのです。
これは間違ったカウントにつながる可能性があり、今度はメモリリークや、使用し終わる前に値がドロップされることにつながる可能性のある潜在的なバグです。 必要なのは、いかにもRc<T>のようだけれども、参照カウントへの変更をスレッドセーフに行うものです。
*/
// スマートポインタ Rc を使用し、1 つの値に対して複数の所有権を与えられるようにする
/*
fn shared_state_concurrency_multi_thread() {
    println!("===== shared_state_concurrency_multi_thread =====");
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter); // Rc::clone で Rc<T> の参照カウントを増やす
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
*/

fn shared_state_concurrency_multi_thread() {
    println!("===== shared_state_concurrency_multi_thread =====");
    let counter = Arc::new(Mutex::new(0));
    println!("counter = {:?}", Arc::strong_count(&counter));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Arc::clone で Arc<T> の参照カウントを増やす
        println!("counter = {:?}", Arc::strong_count(&counter));
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("num = {}", *num);

            // sleep seconds randomly
            let sleep_sec = rand::random::<u64>() % 3;
            thread::sleep(Duration::from_secs(sleep_sec as u64));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter = {:?}", Arc::strong_count(&counter));
    println!("Result: {}", *counter.lock().unwrap());
}
