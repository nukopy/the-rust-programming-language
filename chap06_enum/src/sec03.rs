#[allow(dead_code)]
pub fn main() {
    println!("===== sec03 =====");
    hello_if_let();
}

fn not_if_let() {
    let some_i32_value = Some(55);

    // match の強制された包括性はあるが、コードが冗長
    match some_i32_value {
        Some(55) => println!("fifty five"),
        _ => (),
    }
}

fn if_let() {
    let some_i32_value = Some(55);

    // match の強制された包括性はないが、コードが簡潔
    // 言い換えると、if letは値が一つのパターンにマッチした時にコードを走らせ、他は無視する match への糖衣構文（シンタックスシュガーシュガー）と考えることができる
    if let Some(55) = some_i32_value {
        println!("fifty five");
    }
}

fn if_let_else() {
    let some_i32_value = Some(55);

    if let Some(55) = some_i32_value {
        println!("fifty five");
    } else {
        println!("not fifty five");
    }
}

fn hello_if_let() {
    not_if_let();
    if_let();
    if_let_else();
}
