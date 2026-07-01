# Contrats API

## Statut

Contrats cibles. Ne pas implémenter l'API avant `domain`, `content`, `audit` et `session`.

## Enveloppe standard

```json
{
  "data": {},
  "meta": {
    "request_id": "...",
    "version": "v1"
  }
}
```

Erreur :

```json
{
  "error": {
    "code": "validation_failed",
    "message": "Corpus invalid",
    "details": []
  },
  "meta": {
    "request_id": "..."
  }
}
```

## Endpoints MVP

### `GET /healthz`

Retourne l'état runtime sans secret.

### `GET /v1/catalog`

Liste les parcours publiés.

### `POST /v1/sessions`

Crée une session anonyme ou authentifiée.

```json
{
  "track_id": "ai-practices-basics",
  "locale": "fr-FR",
  "mode": "anonymous"
}
```

### `GET /v1/sessions/{session_id}/next`

Retourne la prochaine question sans exposer la pondération interne complète.

### `POST /v1/sessions/{session_id}/answers`

Soumet une réponse.

```json
{
  "question_id": "q-data-001",
  "choice_ids": ["inspect_content"],
  "client_answered_at": "2026-07-01T12:00:00Z"
}
```

Retourne feedback pédagogique.

### `GET /v1/sessions/{session_id}/summary`

Retourne synthèse utilisateur.

### `GET /v1/admin/audit-report`

Admin seulement. Retourne la qualité du corpus publié.

## Sécurité API

- Sessions anonymes par défaut.
- Auth organisation optionnelle via OIDC.
- Cookies `HttpOnly; Secure; SameSite=Strict`.
- CSRF same-origin.
- Rate limit sur création de session et soumission.
- Pas de texte libre utilisateur en MVP.
- Pas d'export nominatif sans ADR.

## Admin contenu

L'administration de contenu peut rester hors produit au début : git + CLI + revue MR. Une UI admin n'est pas MVP.
