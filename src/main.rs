fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod bank_test {
    use super::*;

    #[test]
    fn new_accounts_can_be_created() {
        create_new_account();
        find_account();
    }
}