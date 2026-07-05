---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../content-governance.md
  - ../question-model.md
  - ../security-rgpd.md
  - ../02-research/bibliography.md
---

# Politique de sources

## Objectif

Définir quelles sources peuvent soutenir les concepts, questions, feedbacks,
fiches et décisions éditoriales de `rumble-ai-practices`, et comment gérer les
incertitudes.

La politique de sources sert à empêcher trois dérives :

1. publier des affirmations non vérifiables ;
2. utiliser des sources faibles comme preuve forte ;
3. laisser une IA inventer ou sur-interpréter des références.

## Principes

- Une source soutient un claim précis, pas une impression générale.
- Le niveau de source doit être proportionné à l'affirmation.
- Une source vendeur peut documenter son produit, mais rarement établir une
  vérité générale.
- Une source interne peut être valide, mais doit être identifiable sans exposer
  de secret.
- L'absence de source robuste doit être visible via `source_gap`.
- Un contenu avec source gap n'est pas publiable sauf justification interne
  explicite et revue humaine.

## Niveaux de sources

| Niveau | Type | Exemples | Usage recommandé | Limites |
| --- | --- | --- | --- | --- |
| A | Normatif/réglementaire | textes officiels, CNIL, AI Act, RGPD, ISO, NIST, ANSSI, ENISA | obligations, contrôles, exigences, définitions réglementaires | dépend du périmètre, pays, secteur, date |
| B | Institutionnel public | agences publiques, autorités, rapports officiels, guides sectoriels | cadrage risque, bonnes pratiques, doctrine publique | peut être général ou non contraignant |
| C | Scientifique | articles évalués, méta-analyses, ouvrages académiques | pédagogie, cognition, évaluation, phénomènes IA | qualité variable, résultats parfois contextuels |
| D | Technique mainteneur | OWASP, MITRE, docs officielles open source, advisories | sécurité, menaces, patterns, vulnérabilités | évolutif, parfois orienté praticien |
| E | Vendeur | OpenAI, Anthropic, Microsoft, Google, éditeurs | capacités, limites déclarées, configuration produit | intérêt commercial, obsolescence rapide |
| F | Politique interne | chartes, classification, doctrine entreprise, procédures | contextualisation organisationnelle | non publique, doit être résumée sans secret |
| G | Retour d'expérience | incident public, postmortem, étude de cas | exemples, signaux faibles, scénarios | anecdotique si non corroboré |

## Sources acceptables

Une source est acceptable si elle est :

- identifiable : titre, auteur ou organisation, date, URL ou référence interne ;
- accessible ou explicitement interne ;
- datée ou versionnée ;
- stable ou archivable autant que possible ;
- reliée à une affirmation précise ;
- comprise dans son périmètre exact ;
- non contradictoire avec une source de niveau supérieur sans explication.

## Sources insuffisantes seules

Les sources suivantes peuvent inspirer une recherche, mais ne suffisent pas pour
une correction publiée :

- réponse générée par IA ;
- billet non daté ;
- thread social ;
- citation de seconde main ;
- benchmark marketing sans méthodologie ;
- capture d'écran sans provenance ;
- article d'opinion sans références ;
- documentation vendeur utilisée pour affirmer une règle générale ;
- contenu inaccessible dont le mainteneur ne peut pas vérifier la portée.

## Hiérarchie en cas de conflit

En cas de conflit apparent, appliquer cette hiérarchie :

1. Politique interne applicable si elle est plus restrictive et compatible avec
   le droit.
2. Texte réglementaire ou autorité compétente applicable au contexte.
3. Standard ou guide reconnu.
4. Littérature scientifique robuste.
5. Documentation fournisseur pour son propre produit.
6. Retour d'expérience ou opinion d'expert.

Le conflit doit être explicité dans le feedback si l'utilisateur peut rencontrer
plusieurs règles selon son organisation.

## Types de claims et niveau minimal

| Type de claim | Exemples | Niveau minimal recommandé |
| --- | --- | --- |
| Obligation juridique | base légale, droits, AI Act, RGPD | A ou F validée par juridique/DPO |
| Bonne pratique sécurité | prompt injection, secrets, permissions agent | A/B/D |
| Capacité fournisseur | option produit, rétention déclarée, API | E, datée et limitée au fournisseur |
| Principe pédagogique | feedback, retrieval practice, charge cognitive | C |
| Doctrine interne | classification, outils autorisés | F |
| Fait technique général | RAG, embeddings, contexte | C/D/E selon affirmation |
| Exemple narratif | scénario plausible | source non obligatoire si fictif, mais revue métier requise |

## Fraîcheur attendue

| Domaine | Revue minimale |
| --- | --- |
| RGPD, AI Act, sécurité, agents, hallucinations | 6 mois |
| Fonctionnalités et politiques fournisseurs IA | 3 à 6 mois |
| Vulnérabilités, prompt injection, tool calling | 3 à 6 mois |
| Pédagogie, sciences cognitives | 12 à 24 mois |
| Concepts techniques stables | 12 mois |
| Politiques internes | selon cycle interne ou 12 mois maximum |
| Médias et provenance | à chaque réutilisation dans un nouveau contexte |

Une source expirée ne rend pas forcément le contenu faux, mais déclenche une
revue.

## Modèle de référence minimal

```yaml
id: source-cnil-ai-001
label: Guide CNIL sur IA et données personnelles
type: regulator_guidance
publisher: CNIL
url: https://...
published_at: YYYY-MM-DD
accessed_at: YYYY-MM-DD
version: null
level: A
domains:
  - privacy_rgpd
supports:
  - claim-minimization-ai-inputs
limitations: Résumer la portée exacte et les limites d'application.
review:
  last_checked_at: YYYY-MM-DD
  reviewer: reviewer-id
```

## Modèle de claim

Pour les contenus complexes, relier source et affirmation par un claim explicite
plutôt qu'une liste générale de références.

```yaml
id: claim-ai-output-needs-verification
statement: Une sortie IA peut être plausible mais factuellement incorrecte ; elle doit être vérifiée avant réutilisation dans un contexte à enjeu.
source_ids:
  - source-nist-ai-rmf-001
  - source-hallucination-survey-001
scope: pratiques professionnelles générales
confidence: high
last_reviewed_at: YYYY-MM-DD
```

## Source gap

Quand une affirmation utile n'a pas encore de source robuste, utiliser un
`source_gap` plutôt qu'une source faible.

```yaml
source_gap:
  reason: source primaire non identifiée
  acceptable_for_status: draft
  reviewer_action: trouver une source A/B/C/D ou réduire l'affirmation
```

Règles :

- autorisé en `draft` ;
- toléré en `review` seulement si le besoin est visible et assigné ;
- bloquant en `approved` sauf justification interne explicite ;
- interdit pour une obligation juridique ou sécurité critique.

## Sources internes

Une source interne peut être utilisée sans exposer son contenu complet si :

- elle a un identifiant stable ;
- son propriétaire est connu ;
- sa portée est décrite ;
- le contenu nécessaire à la correction est résumé sans secret ;
- un relecteur habilité confirme l'interprétation ;
- sa date ou version est indiquée.

Exemple :

```yaml
id: internal-data-classification-policy
label: Politique interne de classification des données
url: internal://data-classification
owner: security-governance
type: internal_policy
level: F
public_summary: Définit les niveaux public/interne/confidentiel/restreint.
reviewer_required: security-rgpd
```

## Sources vendeurs

Les sources vendeurs sont acceptées pour :

- décrire une fonctionnalité du fournisseur ;
- citer une politique publiée par le fournisseur ;
- documenter une limite explicitement déclarée ;
- construire un scénario qui mentionne ce fournisseur.

Elles ne doivent pas être utilisées seules pour :

- affirmer une conformité générale ;
- recommander une solution comme sûre ;
- établir une vérité scientifique ;
- comparer objectivement des produits sans méthodologie indépendante.

## Sources scientifiques

Pour les sources scientifiques :

- préférer méta-analyses, revues systématiques ou travaux largement repris ;
- indiquer si le résultat est expérimental, observationnel ou théorique ;
- éviter de sur-généraliser une étude de laboratoire à tous les contextes
  professionnels ;
- relier la source à une implication pédagogique concrète.

## Sources réglementaires

Pour les sources réglementaires :

- citer le texte ou l'autorité applicable ;
- préciser la juridiction et le périmètre ;
- ne pas transformer une règle générale en conseil juridique individualisé ;
- demander revue DPO/juridique si l'affirmation crée une obligation ou interdit
  une pratique.

## Politique de citation dans les contenus

Un feedback utilisateur ne doit pas devenir une bibliographie lourde. Il doit :

- citer la source la plus utile ;
- expliquer la règle en langage clair ;
- renvoyer vers une source ou politique interne si disponible ;
- signaler quand la règle dépend de l'organisation.

La documentation longue peut contenir les références détaillées ; l'interface
peut présenter des sources synthétiques.

## Revue et retrait de sources

Une source doit être revue si :

- l'URL disparaît ;
- le document change de version ;
- une source de niveau supérieur la contredit ;
- le domaine est arrivé à échéance de revue ;
- un relecteur signale une sur-interprétation.

Une source retirée doit conserver une trace :

```yaml
retired_reason: superseded_by_new_regulator_guidance
retired_at: YYYY-MM-DD
replacement_source_id: source-...
```

## Interdictions

- Inventer une source.
- Citer une source non lue.
- Citer un lien qui ne soutient pas l'affirmation.
- Masquer qu'une source est interne.
- Utiliser une source vendeur comme preuve d'indépendance.
- Faire passer une réponse IA pour une référence.
- Publier un claim juridique sans revue adaptée.

## Checklist auteur

Avant de soumettre un contenu :

1. Chaque affirmation forte a-t-elle une source ?
2. La source soutient-elle précisément cette affirmation ?
3. Le niveau de source est-il adapté au type de claim ?
4. La date de consultation est-elle renseignée ?
5. La source est-elle encore valide ?
6. Les limites de la source sont-elles visibles ?
7. Un `source_gap` est-il utilisé honnêtement si nécessaire ?
8. Le contenu reste-t-il non publiable tant que la source manque ?

## Critères d'acceptation

- Les sources sont classées par niveau.
- Les affirmations fortes ont une source adaptée.
- Les sources vendeurs ne servent pas seules à établir une vérité générale.
- Les dates de consultation sont renseignées.
- Les sources internes sont identifiables sans exposer de secret.
- Les conflits et source gaps ont un traitement explicite.
- La politique est compatible avec le modèle de question existant.

## Statut de revue

Ce document a passé la revue humaine `foundation-review` et sert de référence
canonique pour la Vague A.
