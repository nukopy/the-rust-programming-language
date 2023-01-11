extern crate chap03_variables;

// use chap03_variables::utils; // ここだと mod "scholar_type" 内で utils の名前解決ができない

pub fn main(is_do: bool) {
    if !is_do {
        return;
    }

    println!("===== chap02 =====");
    scholar_types::main();
    compound_types::main();
    println!()
}

mod scholar_types {
    use chap03_variables::utils; // mod 内で use を記述しないと名前解決されない

    pub fn main() {
        println!("--- scholar types ---");
        test_float();
        test_numeric_operations();
        test_bool();
        test_char();
    }

    fn test_float() {
        let x = 2.0; // f64。浮動小数点数の型推論では、デフォルトで f64 が割り当てられる。f32 と比べて速度はほぼ変わらず、より小数が正確に表現できる。
        let y: f32 = 3.0; // f32
        utils::print_type_of(&x);
        utils::print_type_of(&y);
    }

    fn test_numeric_operations() {
        const NUM_PADDING: usize = 16; // quotient_float の文字数が 14 なので padding 幅をそれより多めの 16 に設定

        // addition
        let sum = 5 + 10;
        println!("{:NUM_PADDING$}: {}", "sum", sum);

        // subtraction
        let diff = 95.5 - 4.3;
        println!("{:NUM_PADDING$}: {}", "diff", diff);

        // multiplication
        let product = 4 * 30;
        println!("{:NUM_PADDING$}: {}", "product", product);

        // division
        let quotient_int = 15 / 7;
        let quotient_float = 56.7 / 32.2;

        println!("{:NUM_PADDING$}: {}", "quotient_int", quotient_int);
        println!("{:NUM_PADDING$}: {}", "quotient_float", quotient_float);

        // remainder
        let quotient_int = 43 / 5;
        let remainder = 43 % 5;

        println!("{:NUM_PADDING$}: {}", "quotient_int", quotient_int);
        println!("{:NUM_PADDING$}: {}", "remainder", remainder);
    }

    fn test_bool() {
        let t = true;
        let f: bool = false; // 明示的な型注釈
        println!("t: {}", t);
        println!("f: {}", f);
    }

    fn test_char() {
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻'; //ハート目の猫

        println!("c: {}", c);
        println!("z: {}", z);
        println!("heart_eyed_cat: {}", heart_eyed_cat);
    }
}

mod compound_types {
    use std::io::{self, Write};

    use chap03_variables::utils; // mod 内で use を記述しないと名前解決されない

    pub fn main() {
        println!("--- compound types ---");
        test_tuple();
        test_array();
        test_array_invalid_access();
    }

    fn test_tuple() {
        let tup: (i32, f64, u8) = (500, 6.4, 255);

        // 分配 destructuring
        let (x, y, z) = tup;

        dbg!(tup);
        utils::print_type_of(&tup);
        println!("x: {}", x);
        println!("y: {}", y);
        println!("z: {}", z);

        // 各要素へのアクセス
        println!("x (tup.0): {}", tup.0);
        println!("y (tup.1): {}", tup.1);
        println!("z (tup.2): {}", tup.2);
    }

    fn test_array() {
        let arr = [1, 2, 3, 4, 5];
        dbg!(arr);
        utils::print_type_of(&arr);

        // 明示的な型注釈
        let arr: [f64; 10] = [0.0, 1.1, 2.0, 3.5, 4.3, 5.0, 6.0, 7.0, 8.0, 9.0];
        dbg!(arr);
        utils::print_type_of(&arr);

        // 配列の要素へのアクセス
        let arr = [1, 2, 3, 4, 5, 6];
        let first = arr[0];
        let second = arr[4];
        println!("first: {}", first);
        println!("second: {}", second);

        // let idx = 6; // これはちゃんとコンパイルエラーになる
        // println!("arr[6]: {}", arr[idx]);
    }

    fn test_array_invalid_access() {
        let a = [1, 2, 3, 4, 5];
        dbg!(a);

        loop {
            // input idx
            let mut idx = String::new();

            println!("Please enter an array index.");
            print!("> ");
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut idx) {
                Ok(_) => {
                    println!("User input: \"{}\"", idx.trim());
                }
                Err(error) => {
                    panic!("error: {}", error);
                }
            }

            let idx: usize = match idx.trim().parse() {
                Ok(n) => n,
                Err(error) => {
                    println!("error: {:?}", error);
                    println!("Your input is invalid.\n");

                    continue;
                }
            };

            // a[idx] へのアクセス（もし配列範囲外へのアクセスなら実行時エラー）
            let element = a[idx];
            println!("a[{idx}]: {elem}", idx = idx, elem = element);
            break;
        }
    }
}
