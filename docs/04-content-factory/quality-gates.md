---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../content-governance.md
  - ../testing-strategy.md
  - ../03-domain-model/risk-model.md
  - ../03-domain-model/knowledge-graph.md
  - ./review-process.md
---

# Quality gates éditoriaux

## Objectif

Formaliser les critères qui autorisent, avertissent, échouent ou bloquent un
contenu.

Les gates doivent être compris comme des garde-fous de publication. Ils ne
remplacent pas la revue humaine.

## Niveaux

| Niveau | Effet | Exemple |
| --- | --- | --- |
| `pass` | aucun problème détecté | schéma valide, sources OK |
| `warn` | demande justification/revue | source vendeur seule |
| `fail` | retour auteur obligatoire | feedback incomplet |
| `blocker` | publication interdite | source inventée, PII exposée |

## Principe fail-closed

Si un gate ne peut pas déterminer qu'un contenu est publiable, il ne doit pas le
publier. Il doit produire `warn`, `fail` ou `blocker` selon gravité.

## Gates automatiques

| Gate | Niveau | Condition |
| --- | --- | --- |
| `schema-valid` | fail | YAML/JSON invalide |
| `status-review-consistency` | fail | `approved` sans reviewers/date |
| `source-gap-approved` | blocker | `approved` avec `source_gap` bloquant |
| `no-positive-choice` | fail | aucune réponse positive |
| `missing-feedback` | fail | choix sans feedback |
| `missing-risk` | fail | aucun risque renseigné |
| `missing-competency` | warn/fail | compétence absente selon type contenu |
| `media-review-required` | blocker | média synthétique humain sans revue |
| `retired-served` | blocker | contenu `retired` servi par défaut |
| `expired-review` | warn/fail | date de revue dépassée selon domaine |

## Gates sécurité/RGPD

| Gate | Niveau | Déclencheur |
| --- | --- | --- |
| `secret-exposure` | blocker | token, clé, secret réel ou incitation à l'envoyer |
| `pii-exposure` | blocker | donnée personnelle réelle inutile |
| `unsafe-external-ai-use` | blocker | envoi données sensibles vers outil non autorisé |
| `logs-sensitive` | blocker | logs contenant emails/tokens/IP réelles sans nécessité |
| `rgpd-legal-claim-unsourced` | fail/blocker | claim juridique sans source/revue |
| `retention-undefined` | warn/fail | collecte sans durée de conservation |
| `rh-scoring` | blocker | score nominatif ou leaderboard |

## Gates sources

| Gate | Niveau | Déclencheur |
| --- | --- | --- |
| `invented-source` | blocker | source non vérifiable présentée comme réelle |
| `source-does-not-support-claim` | fail/blocker | claim non soutenu |
| `vendor-only-general-claim` | warn/fail | source vendeur seule pour règle générale |
| `missing-accessed-date` | warn | source web sans date de consultation |
| `expired-source-review` | warn/fail | source périmée |
| `internal-source-with-secret` | blocker | source interne expose secret |

## Gates pédagogiques

| Gate | Niveau | Déclencheur |
| --- | --- | --- |
| `unclear-intent` | fail | intention pédagogique absente |
| `ambiguous-correct-answer` | fail/blocker | bonne réponse indéterminable |
| `trick-question` | fail | piège lexical |
| `feedback-not-actionable` | fail | feedback sans action |
| `dogmatic-correction` | fail | règle absolue non contextualisée |
| `difficulty-mismatch` | warn | difficulté mal calibrée |
| `misconception-unlinked` | warn | distracteur sans misconception claire |

## Gates biais/médias

| Gate | Niveau | Déclencheur |
| --- | --- | --- |
| `stereotype` | fail/blocker | représentation stéréotypée |
| `synthetic-human-unjustified` | blocker | visage/personne IA décorative |
| `media-provenance-missing` | fail | origine/droits absents |
| `bias-review-missing` | fail/blocker | média ou scénario sensible non relu |
| `representation-harm` | blocker | contenu humiliant ou discriminant |

## Gates agent/RAG/sécurité avancée

| Gate | Niveau | Déclencheur |
| --- | --- | --- |
| `excessive-agent-permission` | fail/blocker | droits larges sans contrôle |
| `unsafe-action-without-approval` | blocker | action externe sans validation |
| `indirect-prompt-injection-ignored` | fail/blocker | document traité comme instruction fiable |
| `rag-citation-trusted-blindly` | fail | feedback dit de faire confiance au RAG |
| `tool-output-executed` | blocker | exécution sortie IA sans revue/test |

## Gates par statut

### `draft`

Tolère :

- sources incomplètes ;
- feedback à améliorer ;
- relations conceptuelles partielles.

Ne tolère pas :

- secrets réels ;
- PII réelle ;
- contenu illégal ;
- reproduction propriétaire.

### `review`

Exige :

- schéma valide ;
- intention ;
- feedbacks ;
- risques ;
- sources ou gaps explicites ;
- pas de blocker connu.

### `approved`

Exige :

- revue humaine ;
- sources acceptées ;
- dates ;
- aucun blocker ;
- métadonnées complètes.

## Sortie standard d'un gate

```yaml
gate_id: source-gap-approved
level: blocker
artifact_id: q-source-verify-001
message: Un contenu approved ne peut pas contenir de source_gap bloquant.
evidence:
  field: sources[0].source_gap
action: repasser en draft ou ajouter une source vérifiée
```

## Alignement CLI futur

Les gates doivent pouvoir être traduits en validations déterministes quand c'est
possible. Les gates humains restent dans les rapports.

Commandes candidates :

```bash
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json
```

## Critères d'acceptation

- Chaque gate a un niveau et une justification.
- Les blockers sont fail-closed.
- Les warnings demandent une trace de décision.
- Les gates sont alignés avec les schémas et le CLI.
- Les gates couvrent sources, sécurité/RGPD, pédagogie, biais/médias et RH.
