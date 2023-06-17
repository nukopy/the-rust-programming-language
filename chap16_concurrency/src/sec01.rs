use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn main() {
    println!("===== sec01 =====");
    handle_join_after_for_loop();
    handle_join_before_for_loop();
    move_closure_with_thread();
}

fn handle_join_after_for_loop() {
    println!("===== handle_join_after_for_loop =====");
    let handle = thread::spawn(|| {
        for i in 1..=30 {
            println!("[Spawned] Hi number {} from the spawned thread!", i);
            // thread::sleep を呼び出すと、少々の間、スレッドの実行を止め、違うスレッドを走らせることができる。
            // スレッドはおそらく切り替わるが、切り替わる保証はない。OS がスレッドのスケジュールを行う方法に依る。
            thread::sleep(Duration::from_millis(1));

            // panic を起こしても main スレッドは panic しない。
            // if i == 30 {
            //     panic!("panic!");
            // }
        }
    });

    for i in 1..=5 {
        println!("[Main] Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(5));
    }

    // wait for the spawned thread to finish
    // handle.join().unwrap();
    match handle.join() {
        Ok(_) => println!("Spawned thread finished successfully"),
        Err(_) => println!("Thread panicked"),
    }
}

fn handle_join_before_for_loop() {
    println!("===== handle_join_before_for_loop =====");
    let handle = thread::spawn(|| {
        for i in 1..=30 {
            println!("[Spawned] Hi number {} from the spawned thread!", i);
            // thread::sleep を呼び出すと、少々の間、スレッドの実行を止め、違うスレッドを走らせることができる。
            // スレッドはおそらく切り替わるが、切り替わる保証はない。OS がスレッドのスケジュールを行う方法に依る。
            thread::sleep(Duration::from_millis(1));

            // panic を起こしても main スレッドは panic しない。
            // if i == 30 {
            //     panic!("panic!");
            // }
        }
    });

    // wait for the spawned thread to finish
    // handle.join().unwrap();
    // ここで handle.join() を実行すると、spawn したスレッドが終了するまで、main スレッドは待機する。
    match handle.join() {
        Ok(_) => println!("Spawned thread finished successfully"),
        Err(_) => println!("Thread panicked"),
    }

    for i in 1..=5 {
        println!("[Main] Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(5));
    }
}

fn move_closure_with_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

/*
fn ERROR_move_closure_with_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    drop(v);

    handle.join().unwrap();
}
*/
