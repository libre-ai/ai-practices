---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../02-research/ai-literacy.md
  - ../02-research/cognitive-science.md
  - ./competency-model.md
  - ./risk-model.md
  - ./taxonomy.md
---

# Framework maison de littératie IA

## Objectif

Composer un cadre pédagogique propre à `rumble-ai-practices` à partir de la
recherche et du modèle de compétences.

Le framework doit être utilisable pour écrire des questions, construire des
parcours et produire des synthèses privées, sans devenir une certification RH.

## Nom de travail

**Rumble AI Practice Framework**.

Il structure la littératie IA professionnelle en six dimensions :

1. **Comprendre** — savoir ce qu'une sortie IA est et n'est pas.
2. **Protéger** — préserver données, secrets, accès et droits.
3. **Vérifier** — contrôler sources, faits, citations et sorties.
4. **Cadrer** — formuler une tâche utile sans sur-exposer.
5. **Arbitrer** — adapter l'usage au risque, au contexte et aux politiques.
6. **Assumer** — maintenir responsabilité humaine, documentation et revue.

La sécurité, les biais et les médias traversent ces dimensions et sont explicités
par compétences/risk axes.

## Matrice dimensions / réflexes

| Dimension | Réflexe clé | Questions utilisateur |
| --- | --- | --- |
| Comprendre | sortie IA = proposition | Que puis-je raisonnablement attendre du système ? |
| Protéger | minimiser et choisir outil autorisé | Quelles données ou secrets sont exposés ? |
| Vérifier | remonter à la source | Qu'est-ce qui prouve cette affirmation ? |
| Cadrer | donner contraintes utiles | Comment demander sans créer un nouveau risque ? |
| Arbitrer | proportionner au contexte | Quel est l'impact si la sortie est fausse ? |
| Assumer | garder décision humaine | Qui valide, documente et répond du résultat ? |

## Niveaux pédagogiques

| Niveau | Objectif | Exemple de capacité |
| --- | --- | --- |
| `awareness` | reconnaître | repère qu'un brouillon peut contenir des données internes |
| `guided` | appliquer | choisit d'anonymiser ou d'utiliser un outil autorisé |
| `autonomous` | arbitrer | justifie une escalade DPO/sécurité dans un cas ambigu |
| `mentor` | transmettre | explique à une équipe une règle contextualisée |

## Objectifs observables par dimension

### Comprendre

- Distinguer sortie plausible et preuve.
- Identifier limites de contexte, obsolescence et variabilité.
- Reconnaître que modèle, outil et workflow sont distincts.

### Protéger

- Identifier PII, secrets, documents internes, logs et code sensible.
- Appliquer minimisation, pseudonymisation ou refus.
- Vérifier outil autorisé et politique interne.

### Vérifier

- Identifier les claims importants.
- Vérifier source primaire, date, portée et attribution.
- Signaler l'incertitude quand la preuve manque.

### Cadrer

- Décrire objectif, audience, format et contraintes.
- Fournir contexte utile sans données inutiles.
- Demander limites et hypothèses sans s'y fier aveuglément.

### Arbitrer

- Évaluer impact en cas d'erreur.
- Adapter validation au risque.
- Choisir entre ne pas utiliser, utiliser en brouillon, ou utiliser après revue.

### Assumer

- Maintenir revue humaine pour décisions à impact.
- Documenter sources, hypothèses et limites.
- Escalader au bon rôle.

## Parcours types

### Parcours débutant — réflexes essentiels

1. Sortie IA ≠ vérité.
2. Données personnelles et secrets.
3. Demander une source ≠ vérifier.
4. Feedback et revue humaine.
5. Synthèse privée par axes.

### Parcours intermédiaire — cas métier

1. Documents internes et politiques d'outils.
2. RAG et citations.
3. Code/logs/secrets.
4. Biais et médias.
5. Arbitrage selon impact.

### Parcours avancé — workflows IA

1. Agents et tool calling.
2. Permissions et logs.
3. Prompt injection indirecte.
4. Gouvernance et audit.
5. Documentation de décision.

## Mesure

### Mesure privée utilisateur

- forces par dimension ;
- axes à renforcer ;
- pratiques recommandées ;
- prochains modules.

### Mesure organisationnelle

- agrégats par axe ;
- seuil k-anonyme ;
- pas de score individuel ;
- pas de leaderboard ;
- rétention courte.

## Mapping framework -> compétences

| Dimension | Compétences principales |
| --- | --- |
| Comprendre | `comp-understand-ai-output`, `comp-detect-hallucination` |
| Protéger | `comp-protect-data`, `comp-classify-data-context` |
| Vérifier | `comp-check-source`, `comp-document-decision` |
| Cadrer | `comp-frame-task`, `comp-apply-policy` |
| Arbitrer | `comp-assess-risk`, `comp-escalate` |
| Assumer | `comp-human-accountability`, `comp-secure-agent-tools` |

## Anti-usages du framework

- Classer les collaborateurs.
- Déduire une aptitude professionnelle générale.
- Remplacer une habilitation métier ou sécurité.
- Exporter des faiblesses individuelles.
- Présenter un niveau comme certification légale.

## Critères d'acceptation

- Le framework est utilisable pour écrire des questions.
- Il ne produit pas de classement nominatif.
- Il distingue savoir, action, jugement et responsabilité.
- Il se relie aux compétences et risques canoniques.
- Il permet des parcours sans réponses libres sensibles par défaut.
