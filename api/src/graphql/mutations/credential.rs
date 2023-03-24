use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject};
use hub_core::{producer::Producer, uuid::Uuid};
use ory_openapi_generated_client::models::OAuth2Client;

use crate::{
    graphql::objects::{AccessToken, Credential},
    ory_client::Client,
    proto::{self, credential_events::Event, CredentialEventKey, CredentialEvents},
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "CredentialMutation")]
impl Mutation {
    /// Create an API credential to authenticate and authorize API requests to the Holaplex Hub.
    pub async fn create_credential(
        &self,
        ctx: &Context<'_>,
        input: CreateCredentialInput,
    ) -> Result<CreateCredentialPayload> {
        let AppContext { user_id, .. } = ctx.data::<AppContext>()?;
        let ory = ctx.data::<Client>()?;
        let producer = ctx.data::<Producer<CredentialEvents>>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        // ory client post request payload
        let o_auth2_client = OAuth2Client {
            grant_types: Some(vec!["client_credentials".to_string()]),
            client_name: Some(input.name),
            owner: Some(input.organization.to_string()),
            client_credentials_grant_access_token_lifespan: Some("8760h".to_string()),
            contacts: Some(vec![user_id.to_string()]),
            ..Default::default()
        };

        let o_auth2_client_response = ory.create_client(&o_auth2_client).await?;

        let client_id = o_auth2_client_response
            .client_id
            .clone()
            .ok_or_else(|| Error::new("no client id on OAuth2 client response"))?;

        let client_secret = o_auth2_client_response
            .client_secret
            .clone()
            .ok_or_else(|| Error::new("no client_secret on OAuth2 client response"))?;

        let credential: Credential = o_auth2_client_response.clone().try_into()?;

        let token_exchange_response = ory
            .exchange_token(credential.client_id.clone(), client_secret)
            .await?;

        let access_token = token_exchange_response.try_into()?;

        let event = CredentialEvents {
            event: Some(Event::Oauth2ClientCreated(proto::OAuth2Client {
                user_id: user_id.to_string(),
                client_name: o_auth2_client_response.client_name.unwrap_or_default(),
                organization: input.organization.to_string(),
            })),
        };

        let key = CredentialEventKey {
            id: client_id.to_string(),
            user_id: user_id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(CreateCredentialPayload {
            credential,
            access_token,
        })
    }

    /// Edit the name assigned to the API credential.
    pub async fn edit_credential(
        &self,
        ctx: &Context<'_>,
        input: EditCredentialInput,
    ) -> Result<EditCredentialPayload> {
        let AppContext { user_id, .. } = ctx.data::<AppContext>()?;
        let ory = ctx.data::<Client>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let current_client = ory.get_client(&input.client_id.clone()).await?;
        let current_credential: Credential = current_client.try_into()?;

        // ory client post request payload
        let o_auth2_client = OAuth2Client {
            grant_types: Some(vec!["client_credentials".to_string()]),
            client_name: Some(input.name),
            owner: Some(current_credential.organization_id.clone().to_string()),
            client_credentials_grant_access_token_lifespan: Some("8760h".to_string()),
            contacts: Some(vec![user_id.to_string()]),
            ..Default::default()
        };

        let o_auth2_client_response = ory.update_client(&input.client_id, &o_auth2_client).await?;

        let credential: Credential = o_auth2_client_response.try_into()?;

        Ok(EditCredentialPayload { credential })
    }

    /// Delete the OAuth2 API credential.
    pub async fn delete_credential(
        &self,
        ctx: &Context<'_>,
        input: DeleteCredentialInput,
    ) -> Result<DeleteCredentialPayload> {
        let AppContext { user_id, .. } = ctx.data::<AppContext>()?;
        let ory = ctx.data::<Client>()?;
        let producer = ctx.data::<Producer<CredentialEvents>>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;
        let current_client = ory.get_client(&input.credential.clone()).await?;
        let current_credential: Credential = current_client.clone().try_into()?;

        ory.delete_client(&input.credential).await?;

        let event = CredentialEvents {
            event: Some(Event::Oauth2ClientDeleted(proto::OAuth2Client {
                user_id: user_id.to_string(),
                client_name: current_client.client_name.unwrap_or_default(),
                organization: current_credential.organization_id.to_string(),
            })),
        };

        let key = CredentialEventKey {
            id: input.credential.clone(),
            user_id: user_id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(DeleteCredentialPayload {
            credential: input.credential,
        })
    }
}

/// This struct represents the input for creating a new API credential, including the ID of the organization that the credential will be associated with and the friendly name assigned to the credential.
#[derive(InputObject, Clone, Debug)]
pub struct CreateCredentialInput {
    /// The ID of the organization that the new API credential will be associated with.
    pub organization: Uuid,
    /// The friendly name assigned to the new API credential.
    pub name: String,
}

/// The response payload returned after successfully creating an API credential. It includes the newly created Credential object, which represents the API credential, as well as an `AccessToken` object that can be used to authenticate requests to the Hub API.
#[derive(Debug, Clone, SimpleObject)]
pub struct CreateCredentialPayload {
    /// A `Credential` object representing the newly created API credential.
    credential: Credential,
    /// An `AccessToken` object that can be used to authenticate requests to the Hub API.
    access_token: AccessToken,
}

/// The input for editing the name of an existing credential by providing the `client_id` of the credential and the new `name` to be assigned.
#[derive(InputObject, Clone, Debug)]
pub struct EditCredentialInput {
    /// A unique string identifier assigned to the credential during creation.
    pub client_id: String,
    /// The new name to be assigned to the credential.
    pub name: String,
}

/// The response for editing the name of a credential.
#[derive(Debug, Clone, SimpleObject)]
pub struct EditCredentialPayload {
    /// The updated credential with the edited name.
    credential: Credential,
}

/// The input for deleting a credential.
#[derive(Debug, Clone, InputObject)]
pub struct DeleteCredentialInput {
    /// The unique identifier assigned to the credential to be deleted.
    pub credential: String,
}

/// The response for deleting a credential.
#[derive(Debug, Clone, SimpleObject)]
pub struct DeleteCredentialPayload {
    /// The unique identifier assigned to the deleted credential.
    credential: String,
}
