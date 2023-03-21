use hub_core::clap;
use ory_openapi_generated_client::{
    apis::{
        configuration::Configuration,
        o_auth2_api::{
            create_o_auth2_client, get_o_auth2_client, list_o_auth2_clients, oauth2_token_exchange, delete_o_auth2_client,
            CreateOAuth2ClientError, GetOAuth2ClientError, ListOAuth2ClientsError,
            Oauth2TokenExchangeError, DeleteOAuth2ClientError
        },
        Error,
    },
    models::{OAuth2Client, OAuth2TokenExchange},
};

/// Arguments for establishing a database connection
#[derive(Debug, clap::Args)]
pub struct OryArgs {
    #[arg(long, env, default_value = "http://127.0.0.1:4445")]
    ory_admin_base_url: String,
    #[arg(long, env, default_value = "http://127.0.0.1:4444")]
    ory_public_base_url: String,
    #[arg(long, env, default_value = "")]
    ory_auth_token: String,
}

#[derive(Clone, Debug)]
pub struct Client {
    admin_base_url: String,
    public_base_url: String,
    auth_token: String,
}

impl Client {
    #[must_use]
    pub fn new(args: OryArgs) -> Self {
        let OryArgs {
            ory_admin_base_url,
            ory_public_base_url,
            ory_auth_token,
        } = args;

        Self {
            admin_base_url: ory_admin_base_url,
            public_base_url: ory_public_base_url,
            auth_token: ory_auth_token,
        }
    }

    pub async fn create_client(
        &self,
        o_auth2_client: &OAuth2Client,
    ) -> Result<OAuth2Client, Error<CreateOAuth2ClientError>> {
        let config = Configuration {
            base_path: self.admin_base_url.clone(),
            bearer_access_token: Some(self.auth_token.clone()),
            ..Configuration::default()
        };

        create_o_auth2_client(&config, o_auth2_client).await
    }

    pub async fn get_client(
        &self,
        client_id: &str,
    ) -> Result<OAuth2Client, Error<GetOAuth2ClientError>> {
        let config = Configuration {
            base_path: self.admin_base_url.clone(),
            bearer_access_token: Some(self.auth_token.clone()),
            ..Configuration::default()
        };

        get_o_auth2_client(&config, client_id).await
    }

    pub async fn delete_client(
        &self,
        client_id: &str,
    ) -> Result<(), Error<DeleteOAuth2ClientError>> {
        let config = Configuration {
            base_path: self.admin_base_url.clone(),
            bearer_access_token: Some(self.auth_token.clone()),
            ..Configuration::default()
        };

        delete_o_auth2_client(&config, client_id).await
    }

    pub async fn list_clients(
        &self,
        owner: &str,
        page_size: Option<i64>,
        page_token: Option<&str>,
    ) -> Result<Vec<OAuth2Client>, Error<ListOAuth2ClientsError>> {
        let config = Configuration {
            base_path: self.admin_base_url.clone(),
            bearer_access_token: Some(self.auth_token.clone()),
            ..Configuration::default()
        };

        list_o_auth2_clients(&config, page_size, page_token, None, Some(owner)).await
    }

    pub async fn exchange_token(
        &self,
        client_id: String,
        client_secret: String,
    ) -> Result<OAuth2TokenExchange, Error<Oauth2TokenExchangeError>> {
        let config = Configuration {
            base_path: self.public_base_url.clone(),
            basic_auth: Some((client_id, Some(client_secret))),
            ..Configuration::default()
        };

        oauth2_token_exchange(&config, "client_credentials", None, None, None, None).await
    }
}
