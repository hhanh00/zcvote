use anyhow::Result;
use flutter_rust_bridge::frb;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::runtime::Builder;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    EnvFilter, Layer, Registry,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use zcash_vote2::db::{create_db, list_election_defs, new_election, save_election};

use crate::api::data::Election;

#[frb(opaque)]
pub struct App {
    pool: SqlitePool,
}

impl App {
    #[frb(sync)]
    pub fn new(db_name: &str) -> Result<Self> {
        let connection_options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(db_name);
        let app = std::thread::spawn(|| {
            let r = Builder::new_current_thread().enable_all().build()?;
            r.block_on(async move {
                let pool = SqlitePool::connect_with(connection_options).await?;
                let mut connection = pool.acquire().await?;
                create_db(&mut connection).await?;
                let app = App { pool };
                Ok(app)
            })
        }).join().unwrap();
        app
    }

    pub async fn list_election_defs(&self) -> Result<Vec<Election>> {
        let mut connection = self.pool.acquire().await?;
        Ok(list_election_defs(&mut connection)
            .await
            .map(|r| r.into_iter().map(|e| e.into()).collect())?)
    }

    pub async fn new_election(&self, name: String) -> Result<Election> {
        let mut connection = self.pool.acquire().await?;
        Ok(new_election(&mut connection, name)
            .await
            .map(|r| r.into())?)
    }

    pub async fn save_election(&self, election: Election) -> Result<()> {
        let mut connection = self.pool.acquire().await?;
        Ok(save_election(&mut connection, election.into()).await?)
    }
}

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
    let _ = env_logger::builder().try_init();
    let _ = Registry::default()
        .with(default_layer())
        .with(env_layer())
        .try_init();
    tracing::info!("Rust logging initialized");
}

type BoxedLayer<S> = Box<dyn Layer<S> + Send + Sync + 'static>;

fn default_layer<S>() -> BoxedLayer<S>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fmt::layer()
        .with_ansi(false)
        .with_span_events(FmtSpan::ACTIVE)
        .compact()
        .boxed()
}

fn env_layer<S>() -> BoxedLayer<S>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .boxed()
}
