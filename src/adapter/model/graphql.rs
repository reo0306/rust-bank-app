use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Ping {
    pub status: String,
    pub code: i32,
}