# Paquet 01 — Initialiser le workspace Rust

## Contexte

Le repo est documentation-first. Ce paquet crée le squelette Rust sans logique métier avancée.

## Fichiers autorisés

- `Cargo.toml`
- `crates/*/Cargo.toml`
- `crates/*/src/lib.rs`
- `crates/cli/src/main.rs`
- `.cargo/config.toml` si nécessaire
- `README.md` uniquement pour ajouter les commandes réelles

## Workspace attendu

```text
crates/domain
crates/content
crates/audit
crates/session
crates/api
crates/cli
crates/ui
```

## Dépendances minimales

- `serde`
- `serde_json`
- `thiserror`
- `clap` pour CLI
- `anyhow` seulement dans adapters/CLI, pas dans domain

## Tests

- `cargo fmt --all --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Acceptation

- Workspace compile.
- Chaque crate expose au moins un module vide documenté.
- Pas de dépendance réseau.
- Pas d'UI réelle.

## Non-objectifs

- API HTTP.
- Dioxus app.
- Validation complète du corpus.
