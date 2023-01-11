// 定数の命名規則は、全て大文字でアンダースコアで単語区切り
#[allow(dead_code)]
const MAX_POINTS: u32 = 100_000; // const は値の型注釈が必ず必要

#[allow(dead_code)]
fn get_max_points() -> u32 {
    return MAX_POINTS; // 定数は当然関数の返り値として設定できる
}

pub fn main(is_do: bool) {
    if !is_do {
        return;
    }

    println!("===== chap01 =====");
    test_mutability();
    test_const();
    test_shadowing();
    println!()
}

fn test_mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn test_const() {
    // 以下の定数の定義はコンパイルエラーになる
    // 定数は定数式にしかセットできない。関数呼び出し結果や、実行時に評価される値にはセットできない。
    // const max_points: u32 = get_max_points();
}

fn test_shadowing() {
    let x = 5;
    let x = x + 1;
    println!("The value of x in the outer scope is: {}", x); // 6

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x in the outer scope is: {}", x); // 6

    let mut spaces = "   ";
    println!("spaces: \"{}\"", spaces);
    spaces = "  ";
    println!("spaces: \"{}\"", spaces);
    // spaces = spaces.len(); コンパイルエラーになる。これはシャドーイングではなく、単なる mutable な変数の上書きのため、型エラーになる。
    let spaces = 6; // これはシャドーイング
                    // spaces = "  "; コンパイルエラーになる。直前の行で変数 spaces は新規の i32 型の変数として生成されるため、文字列は受け付けない。
    println!("spaces: \"{}\"", spaces);
}
