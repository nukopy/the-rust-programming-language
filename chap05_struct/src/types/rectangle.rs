#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// 関連関数
impl Rectangle {
    pub fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    #[allow(unused)]
    pub fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// メソッド
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    #[allow(unused)]
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    #[allow(unused)]
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn can_hold(&self, r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Rectangle の関連関数のテスト
     */

    #[test]
    fn test_rectangle() {
        // テスト項目：Rectangle の構造体に渡した値でインスタンスが正しく生成される
        // given (前提条件):
        let width = 30;
        let height = 50;

        // when (操作):
        let r = Rectangle { width, height };

        // then (期待する結果):
        assert_eq!(r.width, width);
        assert_eq!(r.height, height);
    }

    #[test]
    fn test_create_rectangle() {
        // テスト項目: Rectangle::create に渡した値でインスタンスが生成される
        // given (前提条件):
        let width = 30;
        let height = 50;

        // when (操作):
        let r = Rectangle::create(width, height);

        // then (期待する結果):
        assert_eq!(r.width, width);
        assert_eq!(r.height, height);
    }

    #[test]
    fn test_create_square() {
        // テスト項目: Rectangle::create_square に渡した値で正方形が生成される
        // given (前提条件):
        let size = 30;

        // when (操作):
        let r = Rectangle::create_square(size);

        // then (期待する結果):
        assert!(r.width == r.height)
    }

    /* Rectangle のメソッドのテスト
     */

    #[test]
    fn test_area() {
        // テスト項目: area メソッドで面積が正しく計算される
        // given (前提条件):
        let width = 30;
        let height = 50;
        let expected = width * height;
        let r = Rectangle::create(width, height);

        // when (操作):
        let actual = r.area();

        // then (期待する結果):
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set_width() {
        // テスト項目: set_width メソッドで Rectangle.width が更新される
        // given (前提条件):
        let width = 30;
        let height = 50;
        let mut r = Rectangle::create(width, height);
        let passed_width = 100;

        // when (操作):
        assert_eq!(r.width, width);
        r.set_width(passed_width);

        // then (期待する結果):
        assert_eq!(r.width, passed_width);
    }

    #[test]
    fn test_set_height() {
        // テスト項目: set_height メソッドで Rectangle.height が更新される
        // given (前提条件):
        let width = 30;
        let height = 50;
        let mut r = Rectangle::create(width, height);
        let passed_height = 100;

        // when (操作):
        assert_eq!(r.height, height);
        r.set_height(passed_height);

        // then (期待する結果):
        assert_eq!(r.height, passed_height);
    }

    #[test]
    fn test_can_hold() {
        // テスト項目: can_hold メソッドで与えられた Rectangle がはめ込めるかどうか正しく判定できる
        // given (前提条件):
        let width = 30;
        let height = 50;
        let r = Rectangle::create(width, height);

        // params
        let params = [
            (29, 49, true),
            (30, 49, false), // width の境界値
            (29, 50, false), // height の境界値
            (30, 50, false), // width, height の境界値
            (31, 51, false),
        ];

        // when (操作):
        for (w, h, expected) in params {
            let r1 = Rectangle::create(w, h);
            let actual = r.can_hold(&r1);

            // then (期待する結果):
            assert_eq!(actual, expected, "r: {:#?}\nr1: {:#?}", r, r1);
        }
    }
}
