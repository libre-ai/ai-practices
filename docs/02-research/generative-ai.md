---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./llm.md
  - ./hallucinations.md
  - ./cybersecurity.md
  - ../03-domain-model/risk-model.md
---

# IA générative

## Objectif

Présenter les capacités, limites et risques transverses des systèmes d'IA
générative afin d'alimenter les scénarios professionnels.

## Définition de travail

Un système d'IA générative produit ou transforme des contenus : texte, code,
image, audio, vidéo, données structurées ou actions via outils. Il peut aider à
explorer, reformuler, synthétiser ou prototyper, mais ses sorties restent à
vérifier selon le contexte.

Sources candidates : `source-bommasani-foundation-models-2021`,
`source-bender-stochastic-parrots-2021`, sources vendeurs isolées pour les
capacités produit.

## Capacités utiles

| Capacité | Exemples | Risque pédagogique |
| --- | --- | --- |
| Génération texte | brouillon, synthèse, traduction | plausible mais faux, ton trop assuré |
| Génération code | snippets, tests, refactor | vulnérabilité, licence, secret dans prompt |
| Analyse documentaire | résumé, extraction | omission, mauvaise interprétation, PII |
| Idéation | variantes, angles, plan | homogénéisation, biais, manque de vérification |
| Média | image, audio, vidéo | droits, deepfake, stéréotypes, provenance |
| Structuration | tableaux, JSON, classification | format correct mais contenu erroné |
| Tool use | recherche, email, ticket, fichiers | action non voulue, exfiltration, permission excessive |

## Limites communes

- absence de garantie de vérité ;
- dépendance au contexte fourni ;
- obsolescence possible ;
- sensibilité aux instructions et exemples ;
- biais issus des données, prompts ou usages ;
- opacité partielle des mécanismes ;
- variabilité des réponses ;
- difficulté à distinguer certitude et style confiant ;
- risques de confidentialité dans prompts, logs, outils et mémoires.

## Risques professionnels à enseigner

| Risque | Exemple | Réflexe attendu |
| --- | --- | --- |
| Confidentialité | coller un contrat client | anonymiser, outil autorisé, politique interne |
| RGPD | résumer retours utilisateurs identifiants | minimiser, base légale, DPO si doute |
| Hallucination | source inventée | vérifier source primaire |
| Biais | image stéréotypée | auditer représentation, choisir alternative |
| Sécurité | code avec secret | ne pas exposer, scanner, outil interne |
| Responsabilité | décision sensible automatisée | revue humaine, documentation |
| Propriété intellectuelle | réutilisation contenu généré | vérifier droits, politique interne |
| Dépendance fournisseur | fonctionnalité externe | évaluer données, rétention, sortie possible |

## Bonnes pratiques contextualisées

- Décrire la tâche sans données sensibles inutiles.
- Séparer exploration, production et validation.
- Demander incertitudes et hypothèses, mais les vérifier indépendamment.
- Utiliser sources primaires pour les claims importants.
- Garder une trace des arbitrages dans les usages à enjeu.
- Préférer outils autorisés et configurations maîtrisées.
- Refuser l'usage IA quand le risque dépasse les contrôles disponibles.

## Ce qu'il ne faut pas enseigner comme vérité simple

- « L'IA hallucine toujours » : trop vague ; il faut qualifier le risque.
- « Il suffit de demander des sources » : faux sans vérification.
- « Un outil interne est toujours sûr » : dépend des droits, logs et politiques.
- « Un prompt détaillé garantit la qualité » : améliore parfois, ne garantit pas.
- « Les modèles récents règlent les biais » : affirmation à vérifier par contexte.

## Implications corpus

Créer des contenus couvrant :

- texte/code/média/outils ;
- usages exploratoires vs décisions à impact ;
- données publiques, internes, confidentielles, personnelles ;
- vérification source et incertitude ;
- choix de ne pas utiliser l'IA ;
- politiques internes variables.

## Critères d'acceptation

- Ne présente pas l'IA générative comme neutre ou magique.
- Relie chaque limite à un comportement attendu utilisateur.
- Couvre texte, code et médias.
- Isole les sources vendeurs pour les capacités spécifiques.
