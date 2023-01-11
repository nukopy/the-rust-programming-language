pub fn main(is_do: bool) {
    if !is_do {
        return;
    }

    println!("===== chap03 =====");
    test_function();
    test_params_args();
    test_block();
    test_function_with_return_value();
    println!();
}

fn test_function() {
    println!("Hello, world!");
    another_function();
}

// 引数を持たない関数
fn another_function() {
    println!("Another function.")
}

fn test_params_args() {
    let num = 55;
    print_i32(num);

    let val = 101;
    let unit = 'L';
    print_labeled_measurement(val, unit);
}

// 1 つの引数を持つ関数
fn print_i32(x: i32) {
    println!("num of i32: {}", x);
}

// 複数の引数を持つ関数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label);
}

fn test_block() {
    let y = {
        let x = 3;

        // note: x + 1 という式の後にセミコロンを付けるかつけないかでブロックの評価値が i32かユニット () に変わる
        // x + 1;
        x + 1
    };

    println!("y: {:?}", y);
}

fn test_function_with_return_value() {
    println!("five1: {:?}", five1());
    println!("five2: {:?}", five2());
    println!("five3: {:?}", five3());
    println!("five4: {:?}", five4());
}

fn five1() -> i32 {
    5
}

fn five2() -> i32 {
    return 5;
}

// 戻り値がない関数となる
fn five3() -> () {
    5;
}

// 戻り値がない関数なので、そもそも戻り値の型はいらない
fn five4() {
    5;
}
