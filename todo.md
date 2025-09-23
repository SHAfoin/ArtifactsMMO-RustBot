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

Lien des docs de l'API pour chaque fonctions

Structure "joueur" avec le timestamp avant la prochaine action possible
Tokio "sleep_until" pour gérer les délais

Structure pour le player
mis à jour quand des fonctions qui renvoie le caracter avec le font
dans ce cas faire une update

TRIER DANS DES MODULES

Log le nom du character au début de la ligne

Pas besoin de logger le retour des requêtes : visible sur le site
PAR CONTRE logger les programmes en cours ("mode gathering", "mode quête : resoudre XXX quête")

==========================

Automatiser l'amélioration d'arme ? Ou la provoquer mannuellement ? Dans ce cas système de "file d'attente d'action"

Automatiser les tâches par contre
