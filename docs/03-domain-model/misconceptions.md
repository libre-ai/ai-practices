---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../02-research/hallucinations.md
  - ../02-research/cognitive-science.md
  - ../02-research/rag.md
  - ../02-research/agents.md
  - ./risk-model.md
  - ./taxonomy.md
---

# Idées fausses et pièges pédagogiques

## Objectif

Lister les croyances erronées fréquentes à corriger par les scénarios et
feedbacks, sans humilier l'utilisateur.

Une misconception doit être traitée comme un raccourci compréhensible, pas comme
une faute morale. Le feedback doit expliquer pourquoi elle séduit, quel risque
elle crée et quelle action adopter.

## Modèle canonique

```yaml
id: misconception-asking-source-is-verification
statement: Demander à l'IA de citer une source suffit à vérifier une affirmation.
why_seductive: La citation donne une impression de rigueur et fait gagner du temps.
risks:
  - risk-hallucinated-source
  - risk-misattribution
correction: Une source doit être retrouvée et lue pour vérifier qu'elle soutient le claim.
scenario_hooks:
  - citation juridique générée
  - statistique dans une note stratégique
feedback_tone: non_blame
```

## Misconceptions prioritaires

| ID | Croyance erronée | Pourquoi c'est séduisant | Risque | Correction pédagogique |
| --- | --- | --- | --- | --- |
| `misconception-prompt-solves-quality` | Un bon prompt garantit une bonne réponse. | Donne un levier simple. | surconfiance, erreurs non vérifiées | Le prompt aide mais ne remplace pas vérification et revue. |
| `misconception-asking-source-is-verification` | Demander une source suffit. | Citation = apparence de preuve. | source inventée/mauvaise attribution | Lire source primaire et vérifier claim. |
| `misconception-intended-public-means-safe` | Un document destiné à publication est déjà non sensible. | Confond intention et contenu réel. | fuite commentaires, métadonnées, roadmap | Inspecter contenu, métadonnées et classification. |
| `misconception-anonymized-by-removing-name` | Retirer le nom anonymise. | Action visible et rapide. | réidentification | Distinguer anonymisation/pseudonymisation, minimiser. |
| `misconception-internal-tool-always-safe` | Un outil interne est toujours sûr. | Confiance dans l'organisation. | droits/logs/rétention mal compris | Vérifier politique, données, accès et finalité. |
| `misconception-rag-eliminates-hallucinations` | RAG = réponse fiable. | Présence de documents rassure. | mauvaise source, document obsolète | Vérifier citation, version, pertinence. |
| `misconception-long-context-understands-all` | Long contexte = tout est pris en compte. | Capacité technique impressionnante. | omissions critiques | Segmenter, vérifier points clés, demander limites. |
| `misconception-temperature-zero-truth` | Température basse = vérité. | Paramètre simple. | fausse confiance | Variabilité réduite, vérification toujours requise. |
| `misconception-ai-knows-uncertainty` | Le modèle sait quand il ne sait pas. | Ton prudent parfois convaincant. | surconfiance | Traiter incertitude déclarée comme signal, pas preuve. |
| `misconception-agent-is-just-chatbot` | Un agent n'est qu'un assistant texte. | Interface similaire. | actions externes dangereuses | Examiner outils, permissions et validation. |
| `misconception-human-in-loop-solves` | Ajouter un humain suffit. | Responsabilité apparente. | revue symbolique | Définir moment, pouvoir d'arrêt, compétence. |
| `misconception-no-pii-if-business-data` | Données métier ≠ données personnelles. | Catégorie interne rassurante. | PII cachée dans tickets/docs | Inspecter contenu réel et contexte. |
| `misconception-synthetic-media-neutral` | Une image IA est neutre car fictive. | Pas de personne réelle. | stéréotypes, tromperie | Auditer représentation, provenance et utilité. |
| `misconception-score-is-maturity` | Un score élevé prouve la maturité IA. | Mesure simple. | usage RH, illusion compétence | Utiliser axes privés et remédiation. |
| `misconception-ai-review-can-approve` | Une IA de revue peut valider un contenu IA. | Automatisation efficace. | erreurs non détectées, fausse autorité | IA assiste, humain approuve. |
| `misconception-public-source-free-to-use` | Une source publique est librement réutilisable. | Accessibilité confondue avec droits. | droits/licence | Vérifier licence, droits, citation. |
| `misconception-code-generated-is-production-ready` | Code IA qui compile est sûr. | Test superficiel. | vulnérabilités, secrets, licences | Tests, revue sécurité, scan, contexte. |
| `misconception-logs-are-technical-not-sensitive` | Les logs ne sont pas des données sensibles. | Format technique. | tokens, emails, IP, erreurs client | Masquer/minimiser avant partage. |
| `misconception-vendor-says-compliant` | Déclaration fournisseur = conformité de l'usage. | Autorité perçue. | obligation mal couverte | Vérifier contrat, configuration, finalité, DPO. |
| `misconception-bias-only-data` | Le biais vient seulement des données d'entraînement. | Explication connue. | ignore prompt, contexte, usage | Examiner données, modèle, prompt, workflow, revue. |

## Signaux de compréhension partielle

| Signal | Interprétation | Feedback adapté |
| --- | --- | --- |
| « je demande une source » | bonne intuition mais insuffisante | ajouter vérification primaire |
| « j'anonymise » | intention protectrice | distinguer anonymisation/pseudonymisation |
| « outil interne » | pense à l'autorisation | vérifier droits/logs/politique |
| « humain valide » | responsabilité reconnue | préciser compétence et pouvoir d'arrêt |
| « je baisse la température » | comprend variabilité | rappeler factualité indépendante |
| « je fais relire par l'IA » | cherche qualité | rappeler revue humaine et sources |

## Ton de correction

Préférer :

- « C'est une bonne intuition, mais il manque... »
- « Risqué dans ce contexte, car... »
- « La bonne action est de vérifier... »
- « Si la politique interne le permet... »

Éviter :

- « C'est évident » ;
- « Vous auriez dû savoir » ;
- « Faux » sans nuance ;
- humour au détriment de l'utilisateur ;
- formulation qui incite à cacher une erreur.

## Utilisation dans la Content Factory

Chaque nouvelle question peut cibler :

- une compétence ;
- un risque ;
- une misconception principale ;
- un feedback correctif.

Exemple :

```yaml
misconception_ids:
  - misconception-asking-source-is-verification
expected_feedback_pattern: bonne intuition + vérification insuffisante + action source primaire
```

## Critères d'acceptation

- Chaque idée fausse a une correction actionnable.
- Les nuances sont préservées.
- Les formulations évitent le blâme.
- Les misconceptions peuvent être liées à risques, concepts et questions.
- Les corrections ne deviennent pas des slogans dogmatiques.
