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
    /// Get a single API credential by client ID.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The GraphQL context object containing the database connection pool and other data.
    /// * `client_id` - The client ID of the API credential to retrieve.
    ///
    /// # Returns
    ///
    /// The API credential with the specified client ID.
    async fn credential(&self, ctx: &Context<'_>, client_id: String) -> Result<Credential> {
        let ory = ctx.data::<Client>()?;

        let o_auth2_client = ory.get_client(&client_id).await?;

        let credential: Credential = o_auth2_client.try_into()?;

        Ok(credential)
    }

    /// Get a list of API credentials associated with this organization.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The GraphQL context object containing the database connection pool and other data.
    /// * `limit` - Optional limit on the number of credentials to retrieve.
    /// * `offset` - Optional offset for the credentials to retrieve.
    ///
    /// # Returns
    ///
    /// A list of API credentials associated with this organization.
    async fn credentials(
        &self,
        ctx: &Context<'_>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Credential>> {
        let ory = ctx.data::<Client>()?;
        let offset = offset.map(|i| i.to_string());
        let offset = offset.as_deref();

        let o_auth2_clients = ory
            .list_clients(&self.id.to_string(), limit, offset)
            .await?;

        o_auth2_clients
            .into_iter()
            .map(|c| c.try_into().map_err(Into::into))
            .collect::<_>()
    }
}
