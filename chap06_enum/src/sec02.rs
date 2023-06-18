#[allow(dead_code)]
pub fn main() {
    println!("===== sec02 =====");
    hello_enum_match_flow_control();
    extract_value_from_matched_pattern();
    match_option();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            // パターン => コードブロック
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => {
            println!("Not so lucky nickel!");
            5
        }
        Coin::Dime => {
            println!("Not so lucky dime!");
            10
        }
        Coin::Quarter => {
            println!("Not so lucky quarter!");
            25
        }
    }
}

fn hello_enum_match_flow_control() {
    // match の各パターンを通り抜け、値が「適合する」最初のパターンで、値は紐付けられたコードブロックに落ち、実行中に使用される
    let coin = Coin::Penny;
    value_in_cents(coin);
    let coin = Coin::Quarter;
    value_in_cents(coin);
    let coin = Coin::Dime;
    value_in_cents(coin);
    let coin = Coin::Nickel;
    value_in_cents(coin);
}

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

#[derive(Debug)] // すぐに州を点検できるように
enum NextCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_next_cents(coin: NextCoin) -> u32 {
    match coin {
        NextCoin::Penny => {
            // パターン => コードブロック
            println!("Lucky penny!");
            1
        }
        NextCoin::Nickel => {
            println!("Not so lucky nickel!");
            5
        }
        NextCoin::Dime => {
            println!("Not so lucky dime!");
            10
        }
        NextCoin::Quarter(state) => {
            println!("Not so lucky quarter!");
            println!("State quarter from {:?}!", state);
            match state {
                UsState::Alabama => println!("Alabama"),
                UsState::Alaska => println!("Alaska"),
            }

            25
        }
    }
}

fn check_move(coin: NextCoin) {
    match coin {
        // state に coin の所有権を移動する
        NextCoin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => (),
    }

    // パターンにマッチしたとき、値を別の変数に束縛できる（所有権が移動する）ので、以下のコードはコンパイルエラーになる
    // println!("coin: {:?}", coin);
    /* コンパイルエラー
        error[E0382]: borrow of partially moved value: `coin`
      --> src/sec02.rs:99:28
       |
    94 | ...extCoin::Quarter(state) => println!("State quart...
       |                     ----- value partially moved here
    ...
    99 | ...ln!("coin: {:?}", coin);
       |                      ^^^^ value borrowed here after partial move
       |
       = note: partial move occurs because value has type `UsState`, which does not implement the `Copy` trait
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    help: borrow this binding in the pattern to avoid moving the value
       |
    94 |         NextCoin::Quarter(ref state) => println!("State quarter from {:?}!", state),
       |                           +++

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `chap06_enum` due to previous error
         */
}

fn check_not_move(coin: NextCoin) {
    match coin {
        // state に不変参照を束縛する
        NextCoin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => (),
    }

    // coin の所有権は移動していないのでこれはコンパイルが原田亨
    println!(" coin: {:?}", coin);
}

fn extract_value_from_matched_pattern() {
    println!();
    let coin = NextCoin::Penny;
    value_in_next_cents(coin);
    let coin = NextCoin::Nickel;
    value_in_next_cents(coin);
    let coin = NextCoin::Dime;
    value_in_next_cents(coin);
    let coin = NextCoin::Quarter(UsState::Alaska);
    value_in_next_cents(coin);
    let coin = NextCoin::Quarter(UsState::Alabama);
    value_in_next_cents(coin);

    // check move or not
    let coin = NextCoin::Quarter(UsState::Alabama);
    check_move(coin);
    let coin = NextCoin::Quarter(UsState::Alabama);
    check_not_move(coin);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plus_one_map(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}

fn match_placeholder(num: u8) {
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("other"),
        // 下のようにプレースホルダーを使わずにこういう形で束縛することもできる
        // x => println!("Matched {}", x),
    }
}

fn match_option() {
    println!();
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let five = Some(5);
    let six = plus_one_map(five);
    let none = plus_one_map(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    match_placeholder(0u8);
    match_placeholder(155u8);
    match_placeholder(7u8);
}
