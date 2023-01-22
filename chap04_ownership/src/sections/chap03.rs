pub fn main() {
    println!("----- The Book chap04 Ownership - sec03 -----");
    test_slice();
}

fn test_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let hello2 = &s[..5]; // hello と等価
    let world = &s[6..11];
    let world2 = &s[6..];
    let s2 = &s[0..s.len()];
    let s3 = &s[..];

    // assert_eq! は 2 つの引数の式の評価が等価かどうかを見る
    assert_eq!(hello, hello2);
    assert_eq!(world, world2);
    assert_eq!(s, s2);
    assert_eq!(s2, s3);

    println!("&s[0..0]: {}", &s[0..0]);
    println!("&s[0..1]: {}", &s[0..1]);

    let fw = first_word(&s);
    // s.clear();
    /*
    error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
      --> src/sections/chap03.rs:26:5
       |
    7  |     let s = String::from("hello world");
       |         - help: consider changing this to be mutable: `mut s`
    ...
    26 |     s.clear();
       |     ^^^^^^^^^ cannot borrow as mutable
         */
    println!("fw: {}", fw);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
