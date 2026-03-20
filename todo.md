//TODO : Faire des tests unitaires (notamment les fonctions publiques)

- ✅ Requêtes HTTP : reqwest / tokio
  - un thread par joueur, ou pour voir l'inventaire et en attendant bouger...
  - thread d'écoute des events
- ✅ Sérialisation : serde / serde_json
- ✅ Secrets : dotenvy (non) / config / secrecy
  - ✅ secrecy empêche l'affichage des secrets en débug
  - config stocke le timeout / délai?
  - config pour du multi environnement & une structure
  - ✅ avoir un .env.example pour documenter les clés attendues
- ✅ Logging : tracing / tracing_subscriber
  - ✅ faire des spans
  - ✅ INFO, WARN, DEBUG...
- Cache local : sled
  - Voir si il y a une version, sinon timestamp pas trop petit ni trop grand
- ✅~Erreurs : anyhow~ NON au final pas besoin
- API avec Axum
  - démarrer le bot, l'éteindre, voir les logs, état des joueurs, changer de programme...

Bonus :

- Interface : ratatui ?
- Tests : rust basic testing / mockall ?
- CLI : clap ?

=========================

- Pas besoin de logger le retour des requêtes (exemples : tours du combats etc) : visible sur le site ou sur l'endpoint /logs
- PAR CONTRE logger les programmes en cours ("mode gathering", "mode quête : resoudre XXX quête")

==========================

Automatiser l'amélioration d'arme ? Ou la provoquer mannuellement ? Dans ce cas système de "file d'attente d'action"
Automatiser les tâches par contre

==========================

"Simples" tâches à automatiser :

- Fight
- Fight loop
- Crafting

Moyen

- Trouver banque, y aller et déposer/retirer argent/items

Coooooompliqué

- Amélioration d'armes (détecter quand c'est, récupérer le matos, voir si j'ai le niveau...)
- Trouver des bebetes à attaquer & voir si j'ai le niveau/les perfs/le matos pour les battre (et utiliser des potions, et combattre...)
  - commencer en cherchant des monstres de niveau <= au mien
- Taches & quêtes à faire
