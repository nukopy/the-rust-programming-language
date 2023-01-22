mod sections;
mod zenn_rust_book;

fn main() {
    println!("===== The Book chap04 Ownership =====");
    sections::chap01_02::main();
    sections::chap03::main();
    println!();
    zenn_rust_book::main();
}
