# Artefacts MMO

//TODO : Faire des tests unitaires (notamment les fonctions publiques)

- Requêtes HTTP : reqwest / tokio
  - un thread par joueur, ou pour voir l'inventaire et en attendant bouger...
  - thread d'écoute des events
- Sérialisation : serde / serde_json
- Secrets : dotenvy (non) / config / secrecy
  - secrecy empêche l'affichage des secrets en débug
  - config stocke le timeout / délai?
  - config pour du multi environnement & une structure
  - avoir un .env.example pour documenter les clés attendues
- Logging : tracing / tracing_subscriber
  - faire des spans
  - INFO, WARN, DEBUG...
- Cache local : sled
  - Voir si il y a une version, sinon timestamp pas trop petit ni trop grand
- Erreurs : anyhow
- API avec Axum
  - démarrer le bot, l'éteindre, voir les logs, état des joueurs, changer de programme...

Bonus :

- Interface : ratatui ?
- Tests : rust basic testing / mockall ?
- CLI : clap ?

=========================

Structure "joueur" avec le timestamp avant la prochaine action possible
Tokio "sleep_until" pour gérer les délais

Retourner l'erreur avec Anyhow par tout le monde

TRIER DANS DES MODULES

Log le nom du character au début de la ligne

Customiser les logs selon les retours (ex : tache complétée ? si 200 log la récompense)
si erreur : retourner erreur avec le code, log l'erreur (code + msg)
si pas erreur : retourner OK avec le json de la réponse, ajouter un span dans le get/post comme quoi c'est OK, et traiter dans la fonction

tracing_appender pour des logs journaliers

==========================

Automatiser l'amélioration d'arme ? Ou la provoquer mannuellement ? Dans ce cas système de "file d'attente d'action"

Automatiser les tâches par contre
