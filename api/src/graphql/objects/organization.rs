use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;

use super::Credential;
use crate::ory_client::Client;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Organization {
    pub id: Uuid,
}

#[ComplexObject]
impl Organization {
    async fn credential(&self, ctx: &Context<'_>, client_id: String) -> Result<Credential> {
        let ory = ctx.data::<Client>()?;

        let o_auth2_client = ory.get_client(&client_id).await?;

        let credential: Credential = o_auth2_client.try_into()?;

        Ok(credential)
    }

    async fn credentials(
        &self,
        ctx: &Context<'_>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Credential>> {
        let ory = ctx.data::<Client>()?;
        let offset = offset.map(|i| i.to_string());
        let offset = offset.as_ref().map(|x| &**x);

        let o_auth2_clients = ory
            .list_clients(&self.id.to_string(), limit, offset)
            .await?;

        o_auth2_clients
            .into_iter()
            .map(|c| c.try_into().map_err(Into::into))
            .collect::<_>()
    }
}
