pub fn init_logging() -> tracing_appender::non_blocking::WorkerGuard {
    // tracing_subscriber::fmt().with_target(false).init(); // affichage terminal

    let file_appender = tracing_appender::rolling::daily("./logs", "bot.log");
    // Create a non-blocking appender with a background worker. We return the
    // WorkerGuard so the caller can keep it alive until shutdown; dropping
    // the guard will flush the background worker and ensure buffered logs are
    // written before exit.
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_target(false)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    guard
}
