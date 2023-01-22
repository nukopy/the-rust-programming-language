pub fn main() {
    println!("===== Zenn Rust Book =====");
    test_basic_check();
    test_borrow_check();
}

/* immutable な所有権を mutable な仮の所有権として作成することはできない
fn check_ownership() {
    let a = 1;
    let b = &mut a;
}
 */

/* 借用チェック
同一オブジェクトに対する参照と可変について、以下 3 つの制限がある。
これらは関数呼び出し時（かつコンパイル時）にチェックされる。これを借用チェックと呼ぶ。

## 借用チェック

一つのオブジェクトに対して、

1. 不変参照（&）は何個でも同時に存在することができる
2. 不変参照（&）と可変参照（&mut）は同時に存在することができない
3. 可変参照（&mut）は同時に 1 つしか存在することができない
 */

/**
 * 基本的な例
 */

fn test_basic_check() {
    test_basic_check_1();
    test_basic_check_2();
    test_basic_check_3();
    // test_basic_check_4(); // コンパイルエラー
    test_basic_check_5();
    test_basic_check_6();
}

// 所有権「原本、不変」のオブジェクトを変数に束縛
fn test_basic_check_1() {
    let a = 7; // 所有権：原本、不変
    println!("a: {}", a);
}

// 所有権「原本、可変」のオブジェクトを変数に束縛
#[allow(unused_mut)]
fn test_basic_check_2() {
    let mut a = 7; // 所有権：原本、可変
    println!("a: {}", a);
}

// 所有権「原本、不変」のオブジェクトの不変参照を作成する
fn test_basic_check_3() {
    let a: i32 = 7; // 所有権：原本、不変
    let ref_a: &i32 = &a; // 所有権：仮、不変
    println!("a: {}, ref_a: {}", a, ref_a);
}

// 所有権「原本、不変」のオブジェクトの可変参照を作成する（コンパイルエラー）
// -> 不変から可変の参照を作成できない（直感と合ってるね）
// #[allow(unused_variables)]
// fn test_basic_check_4() {
//     let a: i32 = 7; // 所有権：原本、不変
//     let ref_a: &mut i32 = &mut a; // 所有権：仮、可変
// }

/* コンパイル時のエラーメッセージ
訳：a はミュータブルとして宣言されていないので、a を可変として借用することはできません。
error[E0596]: cannot borrow `a` as mutable, as it is not declared as mutable
  --> src/main.rs:62:27
   |
61 |     let a: i32 = 7; // 所有権：原本、不変
   |         - help: consider changing this to be mutable: `mut a`
62 |     let ref_a: &mut i32 = &mut a; // 所有権：仮、可変
   |                           ^^^^^^ cannot borrow as mutable
*/

// 所有権「原本、可変」のオブジェクトの不変参照を作成する
#[allow(unused_mut)]
fn test_basic_check_5() {
    let mut a: i32 = 7; // 所有権：原本、可変
    let ref_a: &i32 = &a; // 所有権：仮、不変
    println!("a: {}, ref_a: {}", a, ref_a);
}

// 所有権「原本、可変」のオブジェクトの可変参照を作成する
// -> 可変から可変の参照を作成できる
#[allow(unused_variables)]
fn test_basic_check_6() {
    let mut a: i32 = 7; // 所有権：原本、可変
    let ref_a: &mut i32 = &mut a; // 所有権：仮、可変
}

// ちなみにこっちはコンパイルエラー（これは println! 側の問題が含まれている）
// fn test_basic_check_6_() {
//     let mut a: i32 = 7; // 所有権：原本、可変
//     let ref_a: &mut i32 = &mut a; // 所有権：仮、可変
//     println!("a: {}, ref_a: {}", a, ref_a);
// }

/* コンパイル時のエラーメッセージ
error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
  --> src/main.rs:80:34
   |
79 |     let ref_a: &mut i32 = &mut a;
   |                           ------ mutable borrow occurs here
80 |     println!("a: {}, ref_a: {}", a, ref_a);
   |                                  ^  ----- mutable borrow later used here
   |                                  |
   |                                  immutable borrow occurs here
   |
*/

/**
 * 借用チェックの例
 */

fn test_borrow_check() {
    test_borrow_check_1();
    // test_borrow_check_2();
    test_borrow_check_3();
    test_borrow_check_4();
    test_borrow_check_7();
}

// 例：1. 不変参照（&）は何個でも同時に存在することができる
fn test_borrow_check_1() {
    let a = 10; // immutable object
    let a_ref1 = &a; // reference
    let a_ref2 = &a; // reference
    println!("{}, {}, {}", a, a_ref1, a_ref2)
}

// コンパイルエラー
/*
fn test_borrow_check_2() {
    let mut a = 10; // mutable object
    let a_ref1 = &a; // reference
    let a_mut_ref1 = &mut a; // mutable reference

    println!("{}, {}", a, a_mut_ref1);
}
*/

// コンパイル通る
#[allow(unused_mut)]
#[allow(unused_variables)]
fn test_borrow_check_3() {
    let mut a = 10; // mutable object
    let a_ref1 = &a; // reference
    let a_mut_ref1 = &mut a; // mutable reference

    // println!("{}, {}", a, a_mut_ref1);
}

#[allow(unused_variables)]
fn test_borrow_check_4() {
    let mut a = 10; // mutable object
    let a_ref1 = &a; // reference
    let a_mut_ref1 = &mut a; // mutable reference
    let a_mut_ref2 = &mut a; // mutable refernece

    // ---------- ここまでは同じ ----------
    // a_mut_ref2 を作成した時点で、a_ref1、a_mut_ref1 が無効となり、存在しないことになる
    *a_mut_ref2 = 20; // assign
    println!("{}", a); // borrow check!! - OK
}

// コンパイルエラーになる
// #[allow(unused_variables)]
// fn test_borrow_check_5() {
//     let mut a = 10; // mutable object
//     let a_ref1 = &a; // reference
//     let a_mut_ref1 = &mut a; // mutable reference
//     let a_mut_ref2 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない
//
//     // ---------- ここまでは同じ ----------
//     // a_mut_ref2 を作成した時点で、a_ref1、a_mut_ref1 が無効となり、存在しないことになるため、コンパイルエラー
//     *a_mut_ref1 = 20; // assign (error)
//     println!("{}", a); // borrow check!! - Error!
// }

// コンパイルエラーになる
// #[allow(unused_variables)]
// fn test_borrow_check_6() {
//     let mut a = 10; // mutable object
//     let a_ref1 = &a; // reference
//     let a_mut_ref1 = &mut a; // mutable reference
//     let a_mut_ref2 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない

//     // ---------- ここまでは同じ ----------
//     // a_mut_ref2 を作成した時点で、a_ref1、a_mut_ref1 が無効となり、存在しないことになるため、a_ref1 は使用できず、コンパイルエラー
//     println!("{}", a_ref1); // borrow check!! - Error!
// }

// -> コンパイルエラー
// #[allow(unused_variables)]
// fn test_borrow_check_6() {
//     let mut a: i32 = 10; // mutable object
//     let a_immut_ref: &i32 = &a; // reference
//     let a_mut_ref: &mut i32 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない

//     // ---------- ここまでは同じ ----------
//     // a_mut_ref を作成した時点で、a_immut_ref が無効となり、存在しないことになるため、a_immut_ref は使用できず、コンパイルエラー
//     println!("{}", a_immut_ref); // borrow check!! - Error!
// }

// これは不変参照と可変参照が同時に存在していることにならない？
// -> ならない。a_immut_ref を作成した時点で、a_mut_ref が無効となり、存在しないことになる。
#[allow(unused_variables)]
fn test_borrow_check_7() {
    let mut a: i32 = 10; // mutable object
    let a_mut_ref: &mut i32 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない
    let a_immut_ref: &i32 = &a; // reference

    // ---------- ここまでは同じ ----------
    println!("{}", a_immut_ref); // borrow check!! - Error!
}

// これは不変参照と可変参照が同時に存在していることにならない？
// -> なる。a_immut_ref を作成した時点で、a_mut_ref が無効となり、存在しないことになる。
// -> コンパイルエラー
// #[allow(unused_variables)]
// fn test_borrow_check_8() {
//     let mut a: i32 = 10; // mutable object
//     let a_mut_ref: &mut i32 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない
//     let a_immut_ref: &i32 = &a; // reference

//     // ---------- ここまでは同じ ----------
//     println!("{}", a_mut_ref); // borrow check!! - Error!
// }

/* メモ：借用チェックに関する自分の理解
不変、可変どちらかの参照を同じスコープ内で作成したときの借用チェックにおいては、
後で作成した参照が存在することになり、それまで作成した参照が無効になる。
そのため、無効になった参照に対してアクセスしようとするとコンパイルエラーになる。
 */

/* 借用チェック
同一オブジェクトに対する参照と可変について，いくつか制限があります．

- 不変参照 (&) は何個でも同時に存在することが出来る
- 不変参照 (&) と可変参照 (&mut) は同時に存在することが出来ない
- 可変参照 (&mut) は同時に１つしか存在することが出来ない

ここで大事なことは，上記の制限は、
関数呼び出し時 （かつコンパイル時）にチェックされるということです（これを 借用チェック と呼びます）．

このチェックが行われる直前の可変参照（必ず１つ）もしくは不変参照（複数可）がその時に存在していることになります．
少なくとも、**可変参照を作成した時には，それまでの不変参照または可変参照がすべて無効となり，存在しないことになります**．
もちろん，あくまで同一オブジェクトに対する参照に対してです．
 */

/*
このように関数呼び出しによる借用チェックによって，
スコープから抜けていない変数であっても，それが参照なら存在していないことになりうるということです
（ここで存在していないと言っていますが，実際には存在できないようにコンパイル時にエラーが出るということ）．
参照を束縛した変数をなるべく作らないことが大切です．
 */
