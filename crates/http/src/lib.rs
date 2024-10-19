use crate::error::Result;
use auth::Claims;
use axum::{extract::State, routing::get, Json, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod auth;
pub mod error;

pub struct Server {
    addr: SocketAddr,
    /// The secret used to sign the JWT tokens.
    jwt_secret: Vec<u8>,
    /// the `iss` value on the JWT
    issuer: String,
}

#[derive(Clone)]
pub struct ServerState {
    pub encoding_key: jsonwebtoken::EncodingKey,
    pub issuer: String,
}

impl Server {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn run(self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(&self.jwt_secret)?;
        let state = ServerState {
            encoding_key,
            issuer: self.issuer,
        };

        let app = Router::new()
            .route("/health", get(health))
            .route("/api/token", get(generate_token))
            .with_state(state)
            .layer(cors);

        tracing::info!("Listening on {0}", self.addr);
        let listener = TcpListener::bind(&self.addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

pub struct Builder {
    addr: Option<SocketAddr>,
    jwt_secret: Option<Vec<u8>>,
    issuer: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            addr: None,
            jwt_secret: None,
            issuer: None,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn jwt_secret(mut self, jwt_secret: Vec<u8>) -> Self {
        self.jwt_secret = Some(jwt_secret);
        self
    }

    pub fn issuer(mut self, issuer: String) -> Self {
        self.issuer = Some(issuer);
        self
    }

    pub fn build(self) -> Result<Server> {
        let addr = self.addr.ok_or(error::Error::ServerBuilder)?;
        let jwt_secret = self.jwt_secret.ok_or(error::Error::ServerBuilder)?;
        let issuer = self.issuer.ok_or(error::Error::ServerBuilder)?;

        Ok(Server {
            addr,
            jwt_secret,
            issuer,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            addr: Some(SocketAddr::from(([0, 0, 0, 0], 5000))),
            jwt_secret: None,
            issuer: None,
        }
    }
}

pub async fn health() -> &'static str {
    "OK"
}

#[axum::debug_handler]
pub async fn generate_token(
    State(ServerState {
        encoding_key,
        issuer,
        ..
    }): State<ServerState>,
) -> Result<Json<String>> {
    let token = Claims::new(issuer).encode(&encoding_key).await?;
    Ok(axum::response::Json(token))
}
