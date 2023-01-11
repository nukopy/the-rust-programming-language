pub fn main(is_do: bool) {
    if !is_do {
        return;
    }

    println!("===== chap05 =====");
    if_expression::main();
    loop_::main();
    while_for::main();
    practice::fizzbuzz();
    println!();
}

// if 式
mod if_expression {
    pub fn main() {
        println!("--- if expression ---");

        let num = 6;

        if num < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        // Rust では truthy / falsy の概念がない。下記のコードはコンパイルエラーになる。
        /*
        if num {
            println!("number was truthy")
        }
        */

        // let 文内で if 式を使う
        let condition = true;
        let num = if condition { 5 } else { 6 };
        println!("num: {}", num);
        let num = if !condition { 5 } else { 6 };
        println!("num: {}", num);
    }
}

mod loop_ {
    pub fn main() {
        println!("--- loop ---");

        // test_infinite_loop();
        test_break_continue();
    }

    fn test_infinite_loop() {
        loop {
            println!("again!");
        }
    }

    fn test_break_continue() {
        let mut count = 0;
        'counting_up: loop {
            println!("[outer loop] count = {}", count);
            let mut remaining = 10;

            loop {
                println!("[inner loop] remaining = {}", remaining);
                if remaining == 8 {
                    println!("🏃🏻‍♂️Escape from inner loop!");
                    break;
                }

                if count == 2 {
                    println!("🏃🏻‍♂️Escape from outer loop!");
                    break 'counting_up;
                }

                remaining -= 1;
            }

            count += 1;
        }
    }
}

mod while_for {
    pub fn main() {
        println!("--- while ---");
        test_while();
        test_for();
    }

    fn test_while() {
        let mut number = 1;

        while number <= 3 {
            println!("{}!", number);

            number += 1;
        }

        println!("ダー");

        // while を用いて配列の要素を列挙する
        let a = [10, 20, 30, 40, 50];
        let mut idx = 0;

        // while idx < 6 { // ちなみにこれは runtime エラーになる
        while idx < a.len() {
            println!("a[{}] = {}", idx, a[idx]);
            idx += 1;
        }
        /* コメント
        しかし、このアプローチは間違いが発生しやすいです;
        添え字の長さが間違っていれば、プログラムはパニックしてしまいます。
        また遅いです。コンパイラが実行時にループの各回ごとに境界値チェックを行うようなコードを追加するからです。
         */
    }

    fn test_for() {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("element: {}", element);
        }

        let range = 0..10;
        println!("for loop with range");
        for num in range {
            println!("{}!", num);
        }
        println!("for loop with range.rev()");
        let range = 0..10; // シャドーイングで宣言し直さないと借用云々でコンパイルエラー
        for num in range.rev() {
            // reverse
            println!("{}!", num);
        }

        println!("for loop with range (minus to plus)");
        let range = -5..5;
        for num in range {
            // reverse
            println!("{}!", num);
        }
    }
}

mod practice {
    pub fn fizzbuzz() {
        let rng = 1..101;
        for num in rng {
            println!("{}", get_fizzbuzz_str(num));
        }
    }

    pub fn get_fizzbuzz_str(n: u32) -> String {
        // 各ブロックの戻り値を式で記述してみた
        if n % 15 == 0 {
            "FizzBuzz".to_string()
        } else if n % 3 == 0 {
            "Fizz".to_string()
        } else if n % 5 == 0 {
            "Buzz".to_string()
        } else {
            n.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    mod practice {
        use crate::sections::sec05::practice;

        #[test]
        fn test_get_fizzbuzz_str() {
            let parameters: [(u32, String); 4] = [
                (3, "Fizz".to_string()),
                (5, "Buzz".to_string()),
                (15, "FizzBuzz".to_string()),
                (11, "11".to_string()),
            ];

            for (n, expected) in parameters {
                let actual = practice::get_fizzbuzz_str(n);
                assert_eq!(expected, actual);
            }
        }
    }
}
