use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;

use crate::graphql::objects::Organization;

#[derive(Default)]
pub struct Query;

#[Object(name = "OrganizationQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_organization_by_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Organization> {
        Ok(Organization { id })
    }
}
