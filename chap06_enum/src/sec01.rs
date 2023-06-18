#[allow(dead_code)]
pub fn main() {
    println!("===== sec01 =====");
    hello_enum();
    enum_with_specific_value();
    all_pattern_of_enum();
    hello_option();
}

#[derive(Debug)]
enum IpAddrKind_ {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind_) {
    println!("ip_kind: {:?}", ip_kind);
}

#[derive(Debug)]
struct IpAddr_ {
    kind: IpAddrKind_,
    address: String,
}

fn hello_enum() {
    let four = IpAddrKind_::V4;
    let six = IpAddrKind_::V6;

    println!("four: {:?}", four);
    println!("six: {:?}", six);
    route(four);
    route(six);

    let home = IpAddr_ {
        kind: IpAddrKind_::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr_ {
        kind: IpAddrKind_::V6,
        address: String::from("::1"),
    };
    println!("home     : {:?}", home);
    println!("home.kind: {:?}", home.kind);
    println!("home.address: {:?}", home.address);

    println!("loopback : {:?}", loopback);
    println!("loopback.kind: {:?}", loopback.kind);
    println!("loopback.address: {:?}", loopback.address);
}

#[derive(Debug)]
enum IpAddr__ {
    V4(String), // enum の列挙子と値の関連付けができる
    V6(String),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // enum の各列挙子と値の型は異なっても良い
    V6(String),
}

fn enum_with_specific_value() {
    // enum の列挙子と値の関連付けができる
    let home = IpAddr__::V4(String::from("127.0.0.1"));
    let loopback = IpAddr__::V6(String::from("::1"));
    println!("home     : {:?}", home);
    println!("loopback : {:?}", loopback);

    // enum の各列挙子と値の型は異なっても良い
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home     : {:?}", home);
    println!("loopback : {:?}", loopback);
}

#[derive(Debug)]
enum Message {
    /*
    このenumには、異なる型の列挙子が4つあります:

    Quitには紐付けられたデータは全くなし。
    Moveは、中に匿名構造体を含む。
    Writeは、単独のStringオブジェクトを含む。
    ChangeColorは、3つのi32値を含む。
     */
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum にはメソッドを定義できる
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("call: Quit!!"),
            Message::Move { x, y } => println!("call: Move!! x: {}, y: {}", x, y),
            Message::Write(s) => println!("call: Write!! s: {}", s),
            Message::ChangeColor(r, g, b) => {
                println!("call: ChangeColor!! r: {}, g: {}, b: {}", r, g, b)
            }
        }
    }
}

fn all_pattern_of_enum() {
    let quit = Message::Quit;
    let move_ = Message::Move { x: 1, y: 2 };
    let write = Message::Write(String::from("hello"));
    let change_color = Message::ChangeColor(1, 2, 3);

    quit.call();
    move_.call();
    write.call();
    change_color.call();
}

/* Rust の初期化処理（prelude）に含まれている
enum Option<T> {
    Some(T),
    None,
}
*/

fn hello_option() {
    // let some_number = Option::Some(5);
    // let some_string = Option::Some("a string");
    // let absent_number: Option<i32> = Option::None;
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // None の場合、型パラメータを指定する必要がある

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // T と Option<T> は異なる方
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // error[E0277]: cannot add `Option<i8>` to `i8`
    let sum = x + y.unwrap(); // unwrap() で Option<T> から T を取り出す
    println!("sum: {}", sum);
}
