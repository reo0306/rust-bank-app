use async_trait::async_trait;
use anyhow::Result;

#[cfg(test)]
use mockall::automock;

fn main() {
    println!("Hello, world!");
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait BankRepository {
    // 新規口座作成
    async fn create_new_account(&self, user_id: &String) -> Result<()>;
    // 残高表示
    async fn find_account(&self, user_id: &String) -> Result<BankAccount>;
    // 履歴
    async fn histories(&self, user_id: &String) -> Result<Option<AccountHistories>>;
    // 入金
    async fn payment(&self, user_id: &String, money: i32) -> Result<()>;
    // 引き落とし
    async fn debit(&self, user_id: &String, money: i32) -> Result<()>;
}

pub struct Bank;
pub struct BankAccount;
pub struct AccountHistories;

#[async_trait]
impl BankRepository for Bank {
    async fn create_new_account(&self, user_id: &String) -> Result<()> {
        println!("user: {}", user_id);
        Ok(())
    }

    async fn find_account(&self, user_id: &String) -> Result<BankAccount> {
        println!("user: {}", user_id);
        Ok(BankAccount)
    }

    async fn histories(&self, user_id: &String) -> Result<Option<AccountHistories>> {
        println!("user: {}", user_id);
        Ok(Some(AccountHistories))
    }

    async fn payment(&self, user_id: &String, money: i32) -> Result<()> {
        println!("user: {}, money: {}", user_id, money);
        Ok(())
    }

    async fn debit(&self, user_id: &String, money: i32) -> Result<()> {
        println!("user: {}, money: {}", user_id, money);
        Ok(())
    }
}

#[cfg(test)]
mod bank_test {
    use mockall::predicate::*;

    use super::*;

    #[tokio::test]
    async fn new_accounts_can_be_created() {
        let mut sut = MockBankRepository::new();

        let user_id = "user123".to_string();

        sut.expect_create_new_account()
            .with(eq(user_id.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let result = sut.create_new_account(&user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn account_can_be_found() {
        let mut sut = MockBankRepository::new();

        let user_id = "user123".to_string();

        sut.expect_find_account()
            .with(eq(user_id.clone()))
            .times(1)
            .returning(|_| Ok(BankAccount));

        let result = sut.find_account(&user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn histories_can_be_found() {
        let mut sut = MockBankRepository::new();

        let user_id = "user123".to_string();

        sut.expect_histories()
            .with(eq(user_id.clone()))
            .times(1)
            .returning(|_| Ok(Some(AccountHistories)));

        let result = sut.histories(&user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn payment_can_be_made() {
        let mut sut = MockBankRepository::new();

        let user_id = "user123".to_string();
        let money: i32 = 10;

        sut.expect_payment()
            .with(eq(user_id.clone()), eq(money))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = sut.payment(&user_id, money).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn debit_can_be_made() {
        let mut sut = MockBankRepository::new();

        let user_id = "user123".to_string();
        let money: i32 = 10;

        sut.expect_debit()
            .with(eq(user_id.clone()), eq(money))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = sut.debit(&user_id, money).await;

        assert!(result.is_ok());
    }
}