pub fn main() {
    println!("----- The Book chap04 Ownership - sec01 ~ sec02 -----");
    test_ownership_and_function();
    test_reference();
    test_borrowing();
}

// コンパイルエラーになる
// error[E0382]: borrow of moved value: `s1`
/*
fn test_string_type_ownership() {
    let s1 = String::from("hello");
    let s2 = s1; // "move" occurs here

    println!("{}, world!", s1);
}
*/

/* コンパイルエラー
error[E0382]: borrow of moved value: `s1`
  --> src/the_book.rs:11:28
   |
8  |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
9  |     let s2 = s1;
   |              -- value moved here
10 |
11 |     println!("{}, world!", s1);
   |                            ^^ value borrowed here after move
   |
 */

fn test_ownership_and_function() {
    let s = String::from("hello"); // sがスコープに入る
    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... s はここではもう有効ではない

    let x = 5; // xがスコープに入る
    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  //

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("takes_ownership: {}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  //

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("makes_copy: {}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn test_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので
  // 何も起こらない

fn test_borrowing() {
    // test_borrowing_immutable_reference();
    test_borrowing_mutable_reference();
    // test_borrowing_two_references();
    test_references();
    test_dangling_references();
}

// コンパイルエラー
// error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
/*
fn test_borrowing_immutable_reference() {
    let s = String::from("hello");
    mutate_string(&s);
}

fn mutate_string(s: &String) {
    s.push_str(", world!");
}
*/
/* エラーメッセージ
error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
  --> src/the_book.rs:74:5
   |
73 | fn mutate_string(s: &String) {
   |                     ------- help: consider changing this to be a mutable reference: `&mut String`
74 |     s.push_str(", world!");
   |     ^^^^^^^^^^^^^^^^^^^^^^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
*/

fn test_borrowing_mutable_reference() {
    let mut s = String::from("hello");
    mutate_string(&mut s);
    println!("{}", s)
}

fn mutate_string(s: &mut String) {
    s.push_str(", world!");
}

// コンパイルエラー
// error[E0499]: cannot borrow `s` as mutable more than once at a time
/*
fn test_borrowing_two_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
*/

/* エラーメッセージ
error[E0499]: cannot borrow `s` as mutable more than once at a time
   --> src/the_book.rs:110:14
    |
109 |     let r1 = &mut s;
    |              ------ first mutable borrow occurs here
110 |     let r2 = &mut s;
    |              ^^^^^^ second mutable borrow occurs here
111 |
112 |     println!("{}, {}", r1, r2);
    |                        -- first borrow later used here
 */

#[allow(unused_variables)]
fn test_references() {
    let mut s = String::from("hello");

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！

    println!("{}", r3);
}

fn test_dangling_references() {
    // let ref_to_nothing = dangle();
    let s = no_dangle();

    println!("{}", s)
}

// コンパイルエラー
// error[E0106]: missing lifetime specifier
/*
fn dangle() -> &String { // dangleはStringへの参照を返す
    let s = String::from("hello"); // s がスコープに入る

    &s // String s への参照を戻り値として返す
} // ここで s はスコープを抜け、ドロップされる。そのメモリは消される
  // 危険
 */

/* エラーメッセージ
error[E0106]: missing lifetime specifier
   --> src/the_book.rs:151:16
    |
151 | fn dangle() -> &String {
    |                ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value,
    but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
    |
151 | fn dangle() -> &'static String {
    |                 +++++++
*/

// 戻り値として所有権を呼び出し元に渡す（ムーブ）
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
