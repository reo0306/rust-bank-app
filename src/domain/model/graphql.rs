use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Ping {
    pub status: String,
    pub code: i32,
}

#[derive(SimpleObject)]
pub struct BankQueryAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}