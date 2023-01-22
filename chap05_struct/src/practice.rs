use crate::types::rectangle::Rectangle;

pub fn main() {
    println!("----- chap02 ~ chap03 -----");

    let width = 40;
    let height = 70;
    let r = Rectangle::create(width, height);

    println!("The area of the rectangle is {} square pixels.", r.area());

    // print Rectangle
    println!("r: {:?}", r);
    println!("r: {:#?}", r); // pretty print // これなんでムーブ起こらないの？

    let r2 = Rectangle::create(30, 25);
    println!(
        "r {} hold r2!\nr: {:#?}\nr2: {:#?}",
        if r.can_hold(&r2) { "can" } else { "cannot" },
        r,
        r2
    );
}
