use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject};
use hub_core::uuid::Uuid;
use ory_openapi_generated_client::models::OAuth2Client;

use crate::{
    graphql::objects::{AccessToken, Credential},
    ory_client::Client,
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "CredentialMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn create_credential(
        &self,
        ctx: &Context<'_>,
        input: CreateCredentialInput,
    ) -> Result<CreateCredentialPayload> {
        let AppContext { user_id, .. } = ctx.data::<AppContext>()?;
        let ory = ctx.data::<Client>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        // ory client post request payload
        let o_auth2_client = OAuth2Client {
            grant_types: Some(vec!["client_credentials".to_string()]),
            client_name: Some(input.name),
            owner: Some(input.organization.to_string()),
            client_credentials_grant_access_token_lifespan: Some("8760h".to_string()),
            audience: Some(
                input
                    .projects
                    .into_iter()
                    .map(|p| {
                        let project = p.to_string();

                        format!("https://holaplex.com/projects/{project}")
                    })
                    .collect(),
            ),
            contacts: Some(vec![user_id.to_string()]),
            scope: Some(input.scopes.join(" ")),
            ..Default::default()
        };

        let o_auth2_client_response = ory.create_client(&o_auth2_client).await?;

        let client_secret = o_auth2_client_response
            .client_secret
            .clone()
            .ok_or_else(|| Error::new("no client_secret on OAuth2 client response"))?;

        let credential: Credential = o_auth2_client_response.try_into()?;

        let token_exchange_response = ory
            .exchange_token(credential.client_id.clone(), client_secret)
            .await?;

        let access_token = token_exchange_response.try_into()?;

        Ok(CreateCredentialPayload {
            credential,
            access_token,
        })
    }
}

#[derive(InputObject, Clone, Debug)]
pub struct CreateCredentialInput {
    pub organization: Uuid,
    pub name: String,
    pub projects: Vec<Uuid>,
    pub scopes: Vec<String>,
}

// Request payload for creating a credential
#[derive(Debug, Clone, SimpleObject)]
pub struct CreateCredentialPayload {
    credential: Credential,
    access_token: AccessToken,
}
