#![allow(clippy::unused_async)]
mod credential;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(credential::Mutation);
