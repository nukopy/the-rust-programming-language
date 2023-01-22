// タプル構造体
pub struct RgbColor(pub i32, pub i32, pub i32);

impl RgbColor {
    // 呼び出し元のスコープで RgbColor インスタンスを使い続けるために self を参照にしている
    pub fn get_rgb_str(&self) -> String {
        let s = format!("rgb({}, {}, {})", self.0, self.1, self.2);

        String::from(s)
    }
}

pub struct Point3d(pub i32, pub i32, pub i32);

impl Point3d {
    // 呼び出し元のスコープで RgbColor インスタンスを使い続けるために self を参照にしている
    pub fn print(&self) {
        println!("(x, y, z) = ({}, {}, {})", self.0, self.1, self.2);
    }
}
