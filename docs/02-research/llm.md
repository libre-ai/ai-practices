---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./generative-ai.md
  - ./hallucinations.md
  - ./evaluation.md
---

# LLM

## Objectif

Documenter les notions LLM nécessaires pour comprendre les limites
opérationnelles sans transformer le produit en cours technique avancé.

## Niveau d'explication attendu

Le produit n'enseigne pas l'entraînement de modèles. Il enseigne les conséquences
pratiques de quelques notions : contexte, tokens, probabilité, variabilité,
entraînement, alignement, outils et évaluation.

## Concepts essentiels

| Concept | Définition utile | Réflexe utilisateur |
| --- | --- | --- |
| Tokenisation | le texte est découpé en unités traitées par le modèle | limites de longueur et coût ne suivent pas les pages exactes |
| Fenêtre de contexte | quantité d'information accessible dans une interaction | un document long peut être mal exploité ou oublié |
| Sampling | choix probabiliste d'une sortie | deux réponses peuvent différer ; vérifier les claims |
| Température | paramètre de variabilité | faible température ≠ vérité garantie |
| Attention/Transformer | mécanisme d'association entre éléments du contexte | ne signifie pas compréhension humaine |
| Fine-tuning | adaptation sur données | peut spécialiser mais pas garantir conformité |
| Instruction tuning/RLHF | adaptation au comportement attendu | peut améliorer utilité, pas éliminer erreurs |
| Distillation/quantization | modèles plus légers | arbitrage qualité/coût/latence |
| MoE | activation partielle d'experts | performance variable selon routage/tâche |

Sources candidates : `source-vaswani-attention-2017`,
`source-brown-gpt3-2020`, `source-kaplan-scaling-laws-2020`.

## Limites opérationnelles

- Les modèles génèrent des suites plausibles, pas une preuve de vérité.
- Le style peut masquer l'incertitude.
- Les connaissances internes au modèle peuvent être obsolètes.
- Le contexte fourni peut contenir instructions malveillantes ou données
  sensibles.
- Les modèles peuvent suivre une instruction dangereuse si les contrôles autour
  sont faibles.
- Les performances varient selon langue, domaine, format et tâche.

## Mythes à corriger

| Mythe | Correction pédagogique |
| --- | --- |
| Plus gros modèle = toujours meilleur | dépend tâche, coût, données, latence, risque |
| Température 0 = vérité | réduit variabilité, pas hallucination |
| Long contexte = tout compris | récupération et attention restent imparfaites |
| Fine-tuning = sécurité | spécialise un comportement, ne remplace pas gouvernance |
| Le modèle sait s'il sait | il peut exprimer une confiance non fiable |
| Le modèle raisonne comme un humain | utile comme métaphore limitée, pas preuve |

## Vocabulaire à enseigner selon niveau

| Niveau | Vocabulaire |
| --- | --- |
| débutant | sortie, prompt, contexte, source, hallucination, vérification |
| intermédiaire | token, fenêtre de contexte, RAG, grounding, variabilité |
| avancé | tool calling, évaluation, fine-tuning, permissions, red teaming |

## Scénarios à générer

- Résumé d'un document trop long avec omission critique.
- Réponse confiante sur une réglementation obsolète.
- Code généré contenant une vulnérabilité subtile.
- Traduction qui modifie une nuance contractuelle.
- Demande de source produisant une référence plausible mais fausse.

## Implications pour feedbacks

- Expliquer la limite en langage opérationnel.
- Ne pas surcharger avec mathématiques.
- Relier le concept à une action : vérifier, réduire contexte, segmenter,
  choisir outil, demander revue.

## Critères d'acceptation

- Chaque concept technique est relié à un risque ou réflexe.
- Le niveau de détail reste adapté au public cible.
- Les limites de généralisation sont indiquées.
- Les sources candidates sont vérifiées avant passage `stable`.
