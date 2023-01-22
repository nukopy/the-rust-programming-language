use crate::types::tuples::{Point3d, RgbColor};
use crate::types::user::User;

pub fn main() {
    println!("----- chap01 -----");
    test_struct_basic();
    test_struct_update_notation();
    test_tuple_struct();
}

/* 構造体の練習
 */

fn test_struct_basic() {
    // インスタンスの作成
    let user1 = User {
        username: String::from("nukopy"),
        email: String::from("nukopy@gma.com"),
        active: true,
        sign_in_count: 1, // インスタンスの作成時、フィールドの順番（active, sign_in_count）の順序は守らなくて良い（守った方が分かりやすいけど）
    };

    User::print_user(&user1);

    // 可変インスタンスの作成
    let mut user1 = User {
        username: String::from("nukopy"),
        email: String::from("nukopy@gma.com"),
        active: true,
        sign_in_count: 1, // インスタンスの作成時、フィールドの順番（active, sign_in_count）の順序は守らなくて良い（守った方が分かりやすいけど）
    };
    user1.email = String::from("another_nukopy@gma.com");
    user1.sign_in_count += 1;
    User::print_user(&user1);
}

fn test_struct_update_notation() {
    let user1 = User::create(String::from("nukopy"), String::from("nukopy@gma.com"));
    User::print_user(&user1);

    // user1 の email のみを更新した状態でインスタンスの作成
    let user1 = User {
        email: String::from("nukopy@out.com"),
        ..user1
    };
    User::print_user(&user1);

    // ↓の書き方はさせてくれない。なるほど。
    // 必ず構造体のインスタンスの初期化時の最後のフィールドに更新記法を記述しないとコンパイルエラーになる。
    /*
    let user3 = User {
        ..user1,
        email: String::from("nukopy@me.com")
    };
    */
}

/* タプル構造体の練習
 */
fn test_tuple_struct() {
    // タプルに名前をつけてコードに意味付けを行うことができる
    let black = RgbColor(0, 0, 0); // rgb(0, 0, 0)
    let origin_3d = Point3d(0, 0, 0);

    println!("{}", black.get_rgb_str());
    origin_3d.print();
}
