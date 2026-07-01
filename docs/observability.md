# Observabilité

## Objectif

Observer la santé produit sans surveiller les personnes.

## Métriques utiles

- Nombre de sessions démarrées/terminées.
- Temps moyen par question.
- Questions abandonnées.
- Axes pédagogiques les plus difficiles en agrégé.
- Erreurs techniques.
- Version du corpus utilisée.
- Nombre de questions bloquées par validation.

## Métriques interdites par défaut

- Classement nominatif.
- Score individuel exporté à un manager.
- Tracking comportemental fin non nécessaire.
- Captures de réponses libres.

## Logs structurés

Exemple :

```json
{
  "level": "info",
  "event": "answer_evaluated",
  "request_id": "...",
  "session_mode": "anonymous",
  "question_id": "q-data-001",
  "corpus_version": "2026.07.01",
  "result_level": "partial"
}
```

Ne jamais inclure : nom, email, texte libre, token, IP complète si non nécessaire.

## Alertes

- Taux d'erreur API élevé.
- Validation corpus échoue sur `main`.
- Question publiée sans revue détectée.
- Média `blocked` référencé par un parcours.
- CSP violée.

## Rapports éditoriaux

Un rapport hebdomadaire peut lister :

- questions ajoutées ;
- questions retirées ;
- audits média en attente ;
- sources à réviser ;
- contenus proches de l'expiration de revue.
