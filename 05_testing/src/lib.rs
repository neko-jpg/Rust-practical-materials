use mockall::automock;
use proptest::prelude::*;

// 1. Mocking: 依存関係の抽象化
#[automock]
pub trait Database {
    fn save_user(&self, username: &str) -> Result<(), String>;
    fn user_exists(&self, username: &str) -> bool;
}

pub struct UserRegistry {
    db: Box<dyn Database>,
}

impl UserRegistry {
    pub fn new(db: Box<dyn Database>) -> Self {
        Self { db }
    }

    pub fn register(&self, username: &str) -> Result<String, String> {
        if self.db.user_exists(username) {
            return Err("User already exists".to_string());
        }
        self.db.save_user(username)?;
        Ok(format!("User {} registered", username))
    }
}

// 2. Property-based Testing: ロジックの堅牢性検証
pub fn multiply(a: i32, b: i32) -> i32 {
    a.saturating_mul(b) // オーバーフローしない乗算
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_success() {
        let mut mock_db = MockDatabase::new();
        // user_exists は false を返し、save_user は Ok を返すように設定
        mock_db.expect_user_exists().return_const(false);
        mock_db.expect_save_user().return_const(Ok(()));

        let registry = UserRegistry::new(Box::new(mock_db));
        assert!(registry.register("alice").is_ok());
    }

    // Proptest: ランダムな入力値でテスト
    proptest! {
        #[test]
        fn test_multiply_does_not_panic(a in any::<i32>(), b in any::<i32>()) {
            // どんなi32の組み合わせでもパニックしないことを保証
            let _ = multiply(a, b);
        }
    }
}
