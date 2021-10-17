fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("Setting up the Runtime needed has failed and cannot be recovered from");

    // Setting up the logging/tracing stuff
    let colored_tracing = std::env::var("RUST_LOG_COLOR").is_ok();
    let tracing_directive_str =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "monitor=info".to_owned());
    let tracing_sub = tracing_subscriber::FmtSubscriber::builder()
        .with_level(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_directive_str.parse().unwrap()),
        )
        .with_ansi(colored_tracing)
        .finish();
    tracing::subscriber::set_global_default(tracing_sub)
        .expect("Setting initial Tracing-Subscriber");

    let cmd_prefix = ""; // TODO

    match cmd_prefix {
        "server" => {
            runtime.block_on(monitor::server());
        }
        "peer" => {
            runtime.block_on(monitor::peer());
        }
        "client" => {
            runtime.block_on(monitor::client());
        }
        unknown => panic!("Unknown Command: {:?}", unknown),
    };
}
