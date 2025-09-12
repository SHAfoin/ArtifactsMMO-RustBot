//TODO : Faire une fonction pour les appels d'api "all" et une autre pour les appels avec "id"
//TODO : Finir d'ajouter les paramètres query
//TODO : Ajouter les contraintes de query & de code (regex, valeur minimale, maximale...)
//TODO : Voir le code de retour des fonctions, est ce qu'il reçoit le 200 ou 404 ? Est ce qu'il faut faire mon propre code de retour ?

- Requêtes HTTP : reqwest / tokio
  - un thread par joueur, ou pour voir l'inventaire et en attendant bouger...
  - thread d'écoute des events
- Sérialisation : serde / serde_json
- Secrets : dotenvy (non) / config / secrecy
  - secrecy empêche l'affichage des secrets en débug
  - config stocke le timeout / délai?
  - config pour du multi environnement & une structure
  - avoir un .env.example pour documenter les clés attendues

```rust

------- config.toml

[api]
url = "https://api.mmo-game.com"
token = "xxx"

[network]
timeout = 5
retries = 3

------- main.rs

#[derive(Debug, Deserialize)]
struct Api {
    url: String,
    token: String,
}

#[derive(Debug, Deserialize)]
struct Network {
    timeout: u64,
    retries: u32,
}

#[derive(Debug, Deserialize)]
struct Settings {
    api: Api,
    network: Network,
}

```

- Logging : tracing / tracing_subscriber
  - faire des spans
  - INFO, WARN, DEBUG...
- Cache local : sled
  - Voir si il y a une version, sinon timestamp pas trop petit ni trop grand
- Erreurs : anyhow

Bonus :

- Interface : ratatui ?
- Tests : rust basic testing / mockall ?
- CLI : clap ?
