use axum::{handler::Handler, middleware::map_response_with_state, Router, routing};
pub use axum::extract::{Path, Query, State};
pub use axum::http::StatusCode;
pub use axum::response::Json;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use config::{AesSettings, NetworkSettings};

pub mod config;
mod encryption;
mod util;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid settings.http_addr specified")]
    InvalidAddr,
}

pub struct Application<S> {
    router: Router<S>,
    state: S,
}

impl Application<()> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Application<()> {
    fn default() -> Self {
        Self {
            router: Router::new(),
            state: (),
        }
    }
}

impl<S: Clone + Send + Sync + 'static> Application<S> {
    pub fn new_with_state(state: S) -> Self {
        Self {
            router: Router::new(),
            state,
        }
    }

    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.router = self
            .router
            .route(path, routing::method_routing::get(handler));
        self
    }

    pub fn serve_dir(mut self, path: &str, dir: &str) -> Self {
        self.router = self.router.nest_service(path, ServeDir::new(dir));
        self
    }

    pub fn with_encryption(mut self, aes_settings: &'static AesSettings) -> Self {
        self.router = self.router.layer(map_response_with_state(
            aes_settings,
            encryption::encrypt_response,
        ));
        self
    }

    pub fn with_logger(mut self) -> Self {
        self.router = self.router.layer(TraceLayer::new_for_http());
        self
    }

    pub async fn serve(self, settings: &NetworkSettings) -> Result<(), Error> {
        let http_addr = settings.http_addr.parse().map_err(|_| Error::InvalidAddr)?;

        axum_server::bind(http_addr)
            .serve(self.router.with_state(self.state).into_make_service())
            .await?;

        Ok(())
    }
}
