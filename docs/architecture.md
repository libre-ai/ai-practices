# Architecture

## Vue responsabilité

```text
content/questions
      |
      v
crates/content  -> valide les fichiers, schémas, statuts de publication
      |
      v
crates/domain   -> invariants : questions, réponses, axes, risques, feedback
      |
      +--> crates/audit    -> rapports qualité, biais, sources, médias
      +--> crates/session  -> parcours, réponses utilisateur, synthèse
              |
              +--> crates/api  -> HTTP Axum, stockage serveur optionnel
              +--> crates/cli  -> client de référence, validation locale
              +--> crates/ui   -> composants Dioxus
                         |
                         +--> apps/web     -> PWA Dioxus/WASM
                         +--> apps/desktop -> shell Tauri différé
                         +--> apps/mobile  -> différé
```

## Règles d'architecture

1. `domain` ne dépend d'aucun adapter.
2. `content` charge et valide, mais ne décide pas l'UX.
3. `audit` produit des findings déterministes, pas des opinions opaques.
4. `session` consomme le domaine et retourne des synthèses sans connaître la base de données.
5. `api`, `cli`, `ui` sont des adapters minces.
6. L'UI ne réimplémente jamais le scoring.
7. Les contrats de question restent sérialisables et versionnés.
8. L'IA générative, si ajoutée, ne peut produire que des brouillons non publiés.

## Flux principal MVP

```text
Question YAML -> validation schema -> validation métier -> session -> réponse -> feedback -> synthèse
```

## Flux audit média

```text
media asset -> metadata -> review checklist -> bias findings -> decision accepted/blocked/rework
```

## Stockage

### Local-first MVP

- Corpus : fichiers versionnés.
- Sessions locales : JSON ou SQLite local si nécessaire.
- Rapports : fichiers déterministes dans `reports/`.

### Serveur futur

- PostgreSQL pour organisations, parcours publiés, agrégats.
- Redis uniquement si sessions live ou jobs asynchrones.
- Object storage S3-compatible souverain pour médias validés.

## Dépendances externes

MVP sans dépendance IA runtime obligatoire. Les modèles peuvent aider à produire des brouillons hors ligne éditoriale, mais le produit publié doit fonctionner sans appel LLM.

## Sécurité par défaut

- Pas de secrets dans le frontend.
- Pas de token en localStorage/sessionStorage.
- Cookies `HttpOnly; Secure; SameSite=Strict` si auth web.
- CSP avant exposition publique.
- Logs sans PII ni réponses libres sensibles.
- Agrégats anonymisés par défaut.
