use anyhow::Result;
use flutter_rust_bridge::frb;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::{self, format::FmtSpan}, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry};
use zcash_vote2::db::{create_db, get};

#[frb(opaque)]
pub struct App {
    pool: SqlitePool,
}

impl App {
    #[frb]
    pub async fn new(db_name: &str) -> Result<Self> {
        let connection_options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(db_name);
        let pool = SqlitePool::connect_with(connection_options).await?;
        let mut connection = pool.acquire().await?;
        create_db(&mut connection).await?;
        let app = App {
            pool
        };
        Ok(app)
    }

    pub async fn test(&self) -> Result<u32> {
        let mut pool = self.pool.acquire().await?;
        let x = get(&mut pool).await?;
        Ok(x)
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
