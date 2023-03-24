use async_graphql::SimpleObject;
use hub_core::{
    anyhow::{Error, Result},
    chrono::{Days, NaiveDateTime, Utc},
    prelude::*,
};
use ory_openapi_generated_client::models::OAuth2TokenExchange;

const SECONDS_IN_A_DAY: i64 = 86400;

/// An access token used to authenticate and authorize access to the Hub API.
#[derive(Debug, Clone, SimpleObject)]
pub struct AccessToken {
    ///  A string representing the access token used to authenticate requests.
    pub access_token: String,
    /// A timestamp indicating when the access token will expire.
    pub expires_at: NaiveDateTime,
    /// A string indicating the type of access token, such as "Bearer".
    pub token_type: String,
}

impl TryFrom<OAuth2TokenExchange> for AccessToken {
    type Error = Error;

    fn try_from(
        OAuth2TokenExchange {
            access_token,
            expires_in,
            token_type,
            ..
        }: OAuth2TokenExchange,
    ) -> Result<Self> {
        let access_token = access_token.ok_or_else(|| anyhow!("no access token"))?;

        let token_type = token_type.ok_or_else(|| anyhow!("no token type"))?;

        let expires_in = expires_in.ok_or_else(|| anyhow!("no expires in"))?;
        let expires_in_days = expires_in
            .checked_div(SECONDS_IN_A_DAY)
            .ok_or_else(|| anyhow!("numeric overflow on expires in days"))
            .and_then(|v| v.try_into().map_err(Into::into))?;

        let expires_at = Utc::now()
            .checked_add_days(Days::new(expires_in_days))
            .map(|dt| dt.naive_utc())
            .ok_or_else(|| anyhow!("issue converting expires in to expires at"))?;

        Ok(Self {
            access_token,
            expires_at,
            token_type,
        })
    }
}
