#![allow(clippy::unused_async)]

mod credentials;
mod organizations;

// Add your other ones here to create a unified Query object
// e.x. Query(SomeQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(credentials::Query, organizations::Query);
