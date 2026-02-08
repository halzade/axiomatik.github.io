use parking_lot::Once;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INIT_TRACING: Once = Once::new();

pub fn config() {
    INIT_TRACING.call_once(|| {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "debug,tower_http=debug".into()),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .with_timer(ChronoLocal::new("%Y-%m-%d_%H:%M:%S%.6f".to_string())),
            )
            .init();
    });
}
