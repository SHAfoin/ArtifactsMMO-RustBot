use tracing_subscriber::{filter::EnvFilter, prelude::*};

pub fn init_logging(
    print_console: bool,
) -> (
    tracing_appender::non_blocking::WorkerGuard,
    tracing_appender::non_blocking::WorkerGuard,
) {
    // tracing_subscriber::fmt().with_target(false).init(); // affichage terminal

    // ============== HTTP LOGS
    let file_appender_http = tracing_appender::rolling::daily("./logs/http", "http.log");
    let file_appender_gameplay =
        tracing_appender::rolling::daily("./logs/gameplay", "gameplay.log");
    // Create a non-blocking appender with a background worker. We return the
    // WorkerGuard so the caller can keep it alive until shutdown; dropping
    // the guard will flush the background worker and ensure buffered logs are
    // written before exit.

    let (non_blocking_http, guard_http) = tracing_appender::non_blocking(file_appender_http);
    let (non_blocking_gameplay, guard_gameplay) =
        tracing_appender::non_blocking(file_appender_gameplay);

    let filter_gameplay = EnvFilter::new("gameplay");
    let filter_http = EnvFilter::new("http");

    let gameplay_file_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_writer(non_blocking_gameplay)
        .with_ansi(false)
        .with_filter(filter_gameplay.clone());

    let gameplay_terminal_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(true)
        .with_filter(filter_gameplay.clone());

    let http_file_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_writer(non_blocking_http)
        .with_ansi(false)
        .with_filter(filter_http.clone());

    let http_terminal_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(true)
        .with_filter(filter_http.clone());

    if print_console {
        tracing_subscriber::registry()
            .with(gameplay_file_layer)
            .with(http_file_layer)
            .with(gameplay_terminal_layer)
            .with(http_terminal_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(gameplay_file_layer)
            .with(http_file_layer)
            .init();
    }

    (guard_http, guard_gameplay)
}

// TODO : créer un second logger pour les logs de gameplay (récolte, combat, etc), renvoyer ce guard dans un tuple avec l'autre guard pour les logs d'API, et les garder tous les deux vivants dans le main.rs
// TODO : ajouter un paramètre bool pour aussi afficher les logs dans la console, et pas seulement dans le fichier (mais pour le terminal, afficher un message qui indique le type de log au début)
// TODO : mettre les logs http/gameplay dans des sous dossiers séparés
// TODO : tester la rotation des logs ? si il peut delete un certain nombre d'anciens logs

// Créer un tracing_subscriber non global puis l'utiliser
/*
let tracing_subscriber_gameplay = tracing_subscriber::fmt()
        .with_target(false)
        .with_writer(non_blocking_gameplay)
        .with_ansi(print_console)
        .finish();

    tracing::subscriber::with_default(tracing_subscriber_gameplay, || {
        info!("Testtt");
    });


*/
