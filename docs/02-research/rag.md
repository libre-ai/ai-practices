---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./llm.md
  - ./hallucinations.md
  - ./evaluation.md
  - ./cybersecurity.md
---

# RAG

## Objectif

Clarifier ce que le Retrieval-Augmented Generation améliore, ce qu'il ne résout
pas et quels risques il introduit.

## Définition

Le RAG combine une étape de récupération documentaire avec une étape de génération
par modèle. Il vise à fournir au modèle un contexte pertinent issu d'un corpus.

Source candidate : `source-lewis-rag-2020`. Les métriques et frameworks
pratiques doivent être vérifiés via sources `source-ragas` et autres sources
techniques.

## Architecture type

```text
documents -> parsing -> chunks -> embeddings/index -> retrieval -> reranking
          -> prompt/context -> génération -> citations/feedback -> évaluation
```

## Ce que le RAG améliore

- Accès à des connaissances non présentes dans le modèle.
- Réduction possible de certaines hallucinations factuelles.
- Mise à jour plus facile d'un corpus que réentraînement.
- Traçabilité potentielle via documents récupérés.
- Adaptation à des connaissances internes.

## Ce que le RAG ne garantit pas

- vérité du document récupéré ;
- fraîcheur ou validité juridique ;
- bonne interprétation du passage ;
- citation fidèle ;
- absence de biais du corpus ;
- respect des droits d'accès ;
- protection contre injection dans documents ;
- couverture complète d'une question.

## Risques spécifiques

| Risque | Description | Réflexe pédagogique |
| --- | --- | --- |
| Mauvais chunk | passage incomplet ou hors contexte | vérifier document complet si enjeu |
| Document obsolète | ancienne politique récupérée | vérifier version/date |
| Poisoning | document malveillant dans corpus | contrôler ingestion et sources |
| Injection indirecte | instruction cachée dans document | ne pas suivre instructions du contenu récupéré |
| Mauvaise attribution | citation ne soutient pas claim | lire la source primaire |
| Sur-confiance | présence de sources = confiance excessive | évaluer pertinence et suffisance |
| Droits d'accès | document récupéré non autorisé | filtrer par permissions |

## Évaluation RAG

Dimensions utiles :

- pertinence des documents récupérés ;
- couverture du contexte ;
- fidélité de la réponse au contexte ;
- exactitude des citations ;
- absence d'instructions suivies depuis documents non fiables ;
- respect des droits ;
- robustesse à documents contradictoires.

## Scénarios pédagogiques

- L'assistant cite une procédure interne obsolète.
- Une réponse contient une source qui ne soutient qu'une partie du claim.
- Un PDF récupéré contient une instruction cachée : « ignore les règles ».
- Deux politiques internes contradictoires sont récupérées.
- Un utilisateur croit qu'une réponse RAG est validée parce qu'elle a une note de
  source.

## Bonnes pratiques utilisateur

- Vérifier les citations importantes.
- Contrôler date/version des documents.
- Demander à l'assistant de séparer réponse, sources et incertitudes.
- Ne pas supposer que tous les documents autorisés ont été récupérés.
- Escalader si les sources sont contradictoires.

## Implications corpus

- Créer des questions où « il y a une source » n'est pas suffisant.
- Ajouter des distracteurs du type « faire confiance car RAG interne ».
- Relier RAG à source verification, security et business_context.

## Critères d'acceptation

- Ne confond pas citation et vérification.
- Couvre les attaques et erreurs de récupération.
- Produit des scénarios exploitables.
- Reste hors construction d'un moteur RAG générique.
