# RAS

[Rocket] + [Askama] + [SeaORM]

Crates dans le workspace :

- `models/` : entités et fonctions associées pour les manipuler dans la BDD
- `migration/` : migrations de la base de données
- `app/` : application web principale

## Migrations

Installer la CLI SeaORM : `cargo install sea-orm-cli`.

Pour générer une migration : `sea migrate generate NOM`

Pour lancer les migrations : `sea migrate up`, ou `cargo run` (les migrations sont lancées quand l'application démarre).

[Rocket]: https://rocket.rs/v0.5-rc/guide/
[Askama]: https://djc.github.io/askama/