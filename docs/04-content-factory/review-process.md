---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../content-governance.md
  - ../01-foundation/contribution-policy.md
  - ../03-domain-model/risk-model.md
  - ./quality-gates.md
---

# Processus de revue

## Objectif

Définir comment un contenu passe de brouillon à publication, avec séparation
claire entre assistance IA, validation automatique et décision humaine.

## Workflow

```text
draft -> review -> approved
          |          |
          v          v
       blocked    retired
```

Transitions fail-closed : en cas de doute sur source, sécurité, biais, média ou
usage RH, rester en `draft` ou passer `blocked`.

## Rôles

| Rôle | Vérifie | Peut bloquer |
| --- | --- | --- |
| Auteur | intention, scénario, sources initiales | non, sauf auto-blocage |
| Agent IA assisté | risques, incohérences, reformulations | non |
| Relecteur métier | réalisme, vocabulaire, contexte | oui |
| Relecteur sécurité/RGPD | PII, secrets, conformité, politiques internes | oui |
| Relecteur biais/média | représentations, stéréotypes, médias | oui |
| Relecteur pédagogique | clarté, feedback, niveau, transfert | oui |
| Mainteneur corpus | cohérence globale, statut final | oui, décide `approved` |

## Conditions de passage `draft` -> `review`

- Schéma valide.
- Intention pédagogique renseignée.
- Concepts, risques et compétences reliés ou justification explicite.
- Sources renseignées ou `source_gap` explicite.
- Feedbacks écrits.
- Aucun blocker automatique connu.
- Assistance IA éventuelle déclarée.

## Conditions de passage `review` -> `approved`

- Revue humaine adaptée réalisée.
- Sources acceptées.
- Pas de contradiction sécurité/RGPD.
- Pas de biais non traité.
- Pas de média IA humain réaliste sans audit.
- Date et reviewers renseignés.
- Mainteneur corpus valide.

## Conditions `blocked`

Passer `blocked` si :

- source inventée ;
- incitation à exposer PII, secrets, logs ou code sensible ;
- correction dogmatique sans contexte ;
- média IA humain réaliste non justifié ;
- ambiguïté rendant la bonne réponse instable ;
- usage RH implicite ;
- claim juridique fort non sourcé ;
- bonne pratique dangereuse.

## Conditions `retired`

Passer `retired` si :

- source invalidée ou obsolète ;
- politique interne changée ;
- contenu remplacé par meilleure version ;
- biais découvert après publication ;
- question devenue instable ;
- média retiré ;
- domaine hors périmètre.

## Revue assistée par IA

Autorisé :

- signaler contradictions ;
- trouver risques non couverts ;
- proposer reformulations ;
- générer checklists ;
- comparer au guide éditorial ;
- préparer une synthèse pour humain.

Interdit :

- approuver ;
- inventer source ;
- masquer incertitude ;
- remplacer DPO/sécurité/juridique ;
- décider qu'un biais est acceptable ;
- publier un contenu.

## Rapport de revue attendu

```text
PASS:
WARN:
FAIL:
BLOCKER:
Sources à vérifier:
Risques RGPD/sécurité:
Risques biais/média:
Recommandations de réécriture:
Décision proposée: non finale
```

## Matrice de revue minimale

| Type de contenu | Métier | Sécurité/RGPD | Biais/média | Pédagogie | Mainteneur |
| --- | --- | --- | --- | --- | --- |
| Question données/PII | recommandé | obligatoire | si scénario sensible | obligatoire | obligatoire |
| Question source/hallucination | recommandé | si données | non sauf média | obligatoire | obligatoire |
| Question agent/sécurité | recommandé | obligatoire | si média | obligatoire | obligatoire |
| Média humain/synthétique | selon usage | recommandé | obligatoire | recommandé | obligatoire |
| Fiche réglementaire | recommandé | obligatoire/DPO | si impact | recommandé | obligatoire |
| Prompt de génération | non | obligatoire | recommandé | obligatoire | obligatoire |

## Preuves à conserver

- diff git ;
- rapport CLI ;
- reviewers ;
- date de revue ;
- décision ;
- raison de blocage ou retrait ;
- sources vérifiées ;
- version du contenu.

## Gestion des désaccords

1. Identifier l'axe : sécurité, qualité, performance, complétude,
   souveraineté/conformité.
2. Sécurité/RGPD/biais/RH bloque par défaut.
3. Revenir aux principes et ADR.
4. Documenter la décision si elle a portée durable.

## Critères d'acceptation

- Les rôles de revue sont distincts et compréhensibles.
- Les transitions de statut sont fail-closed.
- La revue assistée par IA ne peut pas approuver.
- Les critères de blocage couvrent sources, sécurité/RGPD, biais et usage RH.
- Les preuves de revue sont identifiables.
