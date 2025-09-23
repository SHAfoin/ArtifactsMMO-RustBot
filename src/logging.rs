pub fn init_logging() {
    // tracing_subscriber::fmt().with_target(false).init(); // affichage terminal

    let file_appender = tracing_appender::rolling::daily("./logs", "bot.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_target(false)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();
}
