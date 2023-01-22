pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

impl User {}

impl User {
    // 関連関数（Python の static method と同じ感じ）
    pub fn create(username: String, email: String) -> User {
        /* ドメインロジック
        - sign_in_count は 1 で初期化
        - active は 1 で初期化
         */

        const INIT_SIGN_IN_COUNT: u64 = 1;
        const INIT_ACTIVE: bool = true;

        User {
            username,
            email,
            sign_in_count: INIT_SIGN_IN_COUNT,
            active: INIT_ACTIVE,
        }
    }

    pub fn print_user(user: &User) {
        println!(
            "<User>\nemail: {}\nusername: {}\nactive: {}\nsign_in_count: {}\n",
            user.email, user.username, user.active, user.sign_in_count
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (String, String, bool, u64) {
        let username = String::from("nukopy");
        let email = String::from("nukopy@gma.com");
        let active = true;
        let sign_in_count = 1;

        (username, email, active, sign_in_count)
    }

    #[test]
    fn test_user() {
        // テスト項目：構造体 User のインスタンス作成が正しく行われる
        let (username, email, active, sign_in_count) = setup();

        // テストしたい
        let user = User {
            username: username.clone(),
            email: email.clone(),
            active,
            sign_in_count,
        };

        // actual, expected
        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        assert_eq!(user.active, active);
        assert_eq!(user.sign_in_count, sign_in_count);
    }

    #[test]
    fn test_create_user() {
        // テスト項目：User.create 関数でインスタンス作成時、active が true、sign_in_count が 1 で作成される
        let (username, email, _, _) = setup();

        // note: User::create に username、email を直接渡すと move が起き、assert 時に使用できないため、clone（deep copy）をしている
        let user = User::create(username.clone(), email.clone());

        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        assert_eq!(user.active, true); // ドメインロジック
        assert_eq!(user.sign_in_count, 1); // ドメインロジック
    }
}
