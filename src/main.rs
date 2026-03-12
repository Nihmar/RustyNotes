use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("rustynotes=info".parse().unwrap()),
        )
        .init();

    info!("Starting RustyNotes");

    let app = rustynotes::App::new();
    app.run();
}
