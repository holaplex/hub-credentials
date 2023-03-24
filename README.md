# Hub Credentials

Hub credentials is a service responsible for managing OAuth2 API credentials for authenticating with the Holaplex Hub API. It allows users to create, edit, and delete API credentials, and provides access tokens for making requests to the API. The service provides GraphQL APIs to manage credentials associated with an organization.

# Getting Started

```
docker compose up -d
cargo run --bin holaplex-hub-credentials 
```

Visit [http://localhost:3005/playground](http://localhost:3005/playground) to access GraphQL playground.

