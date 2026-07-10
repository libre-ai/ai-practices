---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./llm.md
  - ./rag.md
  - ./evaluation.md
  - ../03-domain-model/misconceptions.md
---

# Hallucinations et erreurs de fidélité

## Objectif

Construire une typologie exploitable des sorties plausibles mais fausses ou non
fidèles.

## Définition opérationnelle

Dans le produit, une hallucination désigne une sortie qui paraît crédible mais
n'est pas fidèle aux faits, aux sources, au contexte ou aux contraintes. Le terme
est utile pédagogiquement, mais doit être décomposé pour éviter les slogans.

Source candidate : `source-ji-hallucination-survey-2023`, à vérifier et compléter
par travaux factuality/faithfulness.

## Typologie

| Type | Description | Exemple | Réflexe attendu |
| --- | --- | --- | --- |
| Fait inventé | affirmation sans base factuelle | date, chiffre, décision inexistante | vérifier source primaire |
| Source inventée | référence inexistante | faux article ou URL | rechercher indépendamment |
| Mauvaise attribution | source réelle mais claim non soutenu | citation hors sujet | lire passage source |
| Omission critique | élément important absent | exception juridique omise | demander limites, vérifier dossier |
| Contradiction interne | réponse incohérente | deux chiffres différents | ne pas publier sans résolution |
| Non-respect contrainte | format, périmètre ou interdiction ignorée | inclut données interdites | contrôler avant usage |
| Raisonnement fallacieux | conclusion non déduite des prémisses | causalité abusive | vérifier logique et données |
| Fabrication de procédure | invente politique interne | règle inexistante | consulter politique réelle |

## Causes fréquentes à expliquer simplement

- Le modèle optimise une sortie plausible, pas une garantie de vérité.
- Le contexte fourni peut être incomplet ou ambigu.
- Les données d'entraînement peuvent être obsolètes ou insuffisantes.
- Les instructions peuvent encourager une réponse même quand l'incertitude est
  forte.
- La récupération documentaire peut fournir un mauvais contexte.
- Le style linguistique peut donner une impression de confiance.

## Détection

| Signal | Action |
| --- | --- |
| source trop précise mais introuvable | rechercher indépendamment |
| chiffre sans méthode | demander méthode puis vérifier source |
| citation qui ne correspond pas au lien | lire source |
| réponse très confiante sur sujet changeant | vérifier date/version |
| absence de limites | demander incertitudes, mais ne pas s'y fier seul |
| contradiction avec politique interne | appliquer politique interne et escalader |

## Mitigation proportionnée

- Sujet faible enjeu : reformuler, demander alternatives, vérifier rapidement.
- Sujet publié : vérifier sources, faire relire.
- Données client/RGPD : appliquer politique interne, minimiser, escalader.
- Décision à impact : validation humaine compétente obligatoire.
- Sécurité/code : tests, revue, secret scanning, analyse vulnérabilités.

## Ce qui ne suffit pas

- Demander « cite tes sources ».
- Demander « sois honnête si tu ne sais pas ».
- Baisser la température.
- Utiliser un modèle plus récent.
- Ajouter RAG sans évaluer les sources.
- Croire une réponse parce qu'elle a un ton prudent.

## Feedbacks types

- « La réponse semble plausible, mais la source ne soutient pas l'affirmation :
  il faut vérifier le passage original avant réutilisation. »
- « Le choix est partiellement correct : demander une source aide, mais ne
  remplace pas la vérification indépendante. »
- « Risqué : le document est interne et la sortie IA ne doit pas être utilisée
  comme preuve sans validation métier. »

## Implications corpus

- Créer des choix où demander une source est seulement partiel.
- Tester la différence entre source absente, source inventée, mauvaise source et
  source insuffisante.
- Relier hallucination à responsabilité humaine, pas seulement fiabilité.

## Critères d'acceptation

- La typologie alimente `misconceptions.md` et les questions source verification.
- Les recommandations restent proportionnées au risque.
- Les sources inventées sont un cas explicite.
- Le document évite la promesse d'élimination complète.
