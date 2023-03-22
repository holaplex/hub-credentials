#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod graphql;
pub mod handlers;
pub mod ory_client;

use hub_core::{
    anyhow::{Error, Result},
    clap,
    prelude::*,
    producer::Producer,
    uuid::Uuid,
};
use poem::{async_trait, FromRequest, Request, RequestBody};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/credential.proto.rs"));
}

use proto::CredentialEvents;

impl hub_core::producer::Message for proto::CredentialEvents {
    type Key = proto::CredentialEventKey;
}

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3002)]
    pub port: u16,

    #[command(flatten)]
    pub ory: ory_client::OryArgs,
}

#[derive(Debug, Clone, Copy)]
pub struct UserID(Option<Uuid>);

impl TryFrom<&str> for UserID {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let id = Uuid::from_str(value)?;

        Ok(Self(Some(id)))
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for UserID {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let id = req
            .headers()
            .get("X-USER-ID")
            .and_then(|value| value.to_str().ok())
            .map_or(Ok(Self(None)), Self::try_from)?;

        Ok(id)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub schema: graphql::schema::AppSchema,
    pub ory: ory_client::Client,
    pub producer: Producer<CredentialEvents>,
}

impl AppState {
    #[must_use]
    pub fn new(
        schema: graphql::schema::AppSchema,
        ory: ory_client::Client,
        producer: Producer<CredentialEvents>,
    ) -> Self {
        Self {
            schema,
            ory,
            producer,
        }
    }
}

pub struct AppContext {
    pub user_id: Option<Uuid>,
}

impl AppContext {
    #[must_use]
    pub fn new(user_id: Option<Uuid>) -> Self {
        Self { user_id }
    }
}
