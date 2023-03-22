use holaplex_hub_credentials::{
    graphql::schema::build_schema,
    handlers::{graphql_handler, health, playground},
    ory_client,
    proto::CredentialEvents,
    AppState,
};
use hub_core::clap;
use poem::{get, listener::TcpListener, middleware::AddData, post, EndpointExt, Route, Server};

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3005)]
    pub port: u16,

    #[command(flatten)]
    pub ory: ory_client::OryArgs,
}

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-credentials",
    };

    hub_core::run(opts, |common, args| {
        let Args { port, ory } = args;

        common.rt.block_on(async move {
            let schema = build_schema();

            let ory = ory_client::Client::new(ory);
            let producer = common.producer_cfg.build::<CredentialEvents>().await?;

            let state = AppState::new(schema, ory, producer);

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .at(
                            "/graphql",
                            post(graphql_handler).with(AddData::new(state.clone())),
                        )
                        .at("/playground", get(playground))
                        .at("/health", get(health)),
                )
                .await
                .map_err(Into::into)
        })
    });
}
