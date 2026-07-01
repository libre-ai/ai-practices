# Délégation à des modèles légers

## Objectif

Préparer le repo pour que des modèles moins coûteux puissent implémenter des paquets sans prendre de décisions produit ou sécurité.

## Ce qui est délégable

- Squelettes de crates.
- Types Rust décrits précisément.
- Tests unitaires à partir de cas fournis.
- CLI simple.
- Sérialisation/désérialisation.
- Composants UI sans logique métier.
- Documentation de commandes.

## Ce qui n'est pas délégable sans revue forte

- Décisions d'architecture.
- Choix de dépendances.
- Politique RGPD.
- Scoring organisationnel.
- Publication de contenu.
- Validation de médias IA.
- Authentification/autorisation.
- Déploiement production.

## Format de paquet idéal

Chaque tâche envoyée à un modèle léger doit inclure :

```text
Contexte:
Fichiers à lire:
Fichiers autorisés:
Interfaces exactes:
Cas de tests:
Commandes de vérification:
Non-objectifs:
```

## Garde-fous prompt

- Ne pas demander "propose une architecture".
- Demander "implémente exactement cette interface".
- Fournir les tests attendus.
- Interdire les dépendances supplémentaires sauf validation.
- Demander une sortie courte avec commandes exécutées.

## Exemple

```text
Implémente `QuestionId` dans `crates/domain`.
Lis `docs/implementation/02-domain-contracts.md`.
N'ajoute aucune dépendance.
Ajoute tests: accepte `q-data-001`, refuse `data-001`.
Lance `cargo test -p rumble-ai-practices-domain`.
```

## Review obligatoire

Toute production d'un modèle léger doit être relue sur :

- sécurité ;
- respect des frontières ;
- tests réellement exécutés ;
- absence de logique inventée ;
- absence de dérive de stack.
