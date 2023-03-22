use async_graphql::SimpleObject;
use hub_core::{
    anyhow::{Error, Result},
    chrono::NaiveDateTime,
    prelude::anyhow,
    uuid::Uuid,
};
use ory_openapi_generated_client::models::OAuth2Client;

#[derive(Debug, Clone, SimpleObject)]
pub struct Credential {
    pub name: String,
    pub client_id: String,
    pub scopes: Vec<String>,
    pub audiences: Vec<String>,
    pub created_by_id: Uuid,
    pub organization_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl TryFrom<OAuth2Client> for Credential {
    type Error = Error;

    fn try_from(
        OAuth2Client {
            client_id,
            client_name,
            scope,
            contacts,
            owner,
            created_at,
            audience,
            ..
        }: OAuth2Client,
    ) -> Result<Self> {
        let client_id = client_id.ok_or_else(|| anyhow!("no client id"))?;

        let name = client_name.ok_or_else(|| anyhow!("no client name"))?;

        let audiences = audience.unwrap_or(Vec::new());

        let scopes = scope.map_or(Vec::new(), |s| {
            s.split(' ').map(ToString::to_string).collect()
        });

        let created_by = contacts.ok_or_else(|| anyhow!("no contact list"))?;
        let created_by = created_by.first().ok_or_else(|| anyhow!("no contact"))?;
        let created_by_id = Uuid::parse_str(created_by)?;

        let organization_id = owner.ok_or_else(|| anyhow!("no owner"))?;
        let organization_id = Uuid::parse_str(&organization_id)?;

        let created_at = created_at.ok_or_else(|| anyhow!("no created_at"))?;
        let created_at = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%dT%H:%M:%SZ")?;

        Ok(Self {
            name,
            client_id,
            scopes,
            audiences,
            created_by_id,
            organization_id,
            created_at,
        })
    }
}
