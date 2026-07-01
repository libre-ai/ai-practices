# Paquet 00 — Mode opératoire agents

## Objectif

Permettre à un modèle léger d'implémenter sans rediscuter le produit.

## Règles absolues

- Lire `README.md`, `AGENTS.md`, `docs/architecture.md`, `docs/testing-strategy.md` avant code.
- Ne pas modifier les ADR sans demande explicite.
- Ne pas ajouter de dépendance majeure sans doc dans une ADR ou note technique.
- Ne pas implémenter d'UI avant les crates domaine/contenu/session.
- Ne pas introduire d'appel LLM runtime dans le MVP.
- Ne pas publier de question `approved` automatiquement.

## Format de rendu attendu

Chaque agent doit finir par :

```text
Fichiers changés
Commandes exécutées
Résultat des tests
Risques restants
Prochaine étape recommandée
```

## Priorités de décision

1. Sécurité/RGPD.
2. Qualité et invariants.
3. Performance.
4. Complétude.

## Si ambiguïté

- Choisir l'option la plus sûre.
- Ajouter un TODO explicite.
- Ne pas inventer une politique produit.
- Demander validation humaine.
