use async_graphql::{Context, Object, Result};

use crate::{graphql::objects::Credential, ory_client::Client};

#[derive(Default)]
pub struct Query;

#[Object(name = "CredentialQuery")]
impl Query {
    #[graphql(entity)]
    async fn find_credential_by_client_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] client_id: String,
    ) -> Result<Credential> {
        let ory = ctx.data::<Client>()?;

        let o_auth2_client = ory.get_client(&client_id).await?;
        let credential: Credential = o_auth2_client.try_into()?;

        Ok(credential)
    }
}
