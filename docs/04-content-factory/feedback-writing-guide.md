---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../02-research/cognitive-science.md
  - ../01-foundation/source-policy.md
  - ../03-domain-model/risk-model.md
  - ../03-domain-model/misconceptions.md
  - ./editorial-guide.md
---

# Guide d'écriture des feedbacks

## Objectif

Rendre les corrections utiles, nuancées et actionnables.

Un feedback n'est pas une sanction. C'est l'élément principal d'apprentissage.

## Formule de base

```text
Qualification + risque/bénéfice + raison contextualisée + action recommandée + source/limite
```

Exemple :

> Partiel. Demander une source est une bonne intuition, mais ce n'est pas une
preuve : la source peut être inventée ou mal attribuée. Ouvrez la source primaire
et vérifiez qu'elle soutient précisément l'affirmation.

## Niveaux de feedback

| Niveau | Usage | Ton |
| --- | --- | --- |
| `correct` | action recommandée | confirmer + expliquer pourquoi |
| `partial` | bonne intuition incomplète | valoriser puis compléter |
| `risky` | risque réel | expliquer conséquence et action sûre |
| `incorrect` | choix faux sans danger majeur | corriger sans blâme |
| `blocker` | pratique dangereuse | dire de stopper/escalader clairement |

## Feedback correct

Structure :

1. confirmer ;
2. nommer le risque évité ;
3. expliquer la règle ;
4. proposer la prochaine pratique.

Exemple :

> Correct. Vérifier le contenu réel du document évite d'exposer des commentaires,
versions ou métadonnées internes. Avant tout usage IA, contrôlez la classification
et retirez les informations non nécessaires.

## Feedback partiel

Structure :

1. reconnaître l'intuition ;
2. dire ce qui manque ;
3. préciser l'action complète.

Exemple :

> Partiel. Utiliser un outil autorisé réduit le risque, mais cela ne dispense pas
de minimiser les données. Retirez les informations personnelles non nécessaires
avant l'envoi.

## Feedback risqué

Structure :

1. signaler le risque sans humilier ;
2. expliquer la conséquence ;
3. proposer l'alternative sûre.

Exemple :

> Risqué. Un log peut contenir tokens, emails ou identifiants techniques. Ne le
collez pas tel quel dans un assistant IA : masquez les secrets et utilisez un
outil autorisé ou une procédure interne.

## Feedback blocker

Structure :

1. dire d'arrêter ;
2. nommer le risque critique ;
3. escalader.

Exemple :

> À bloquer. Ce choix expose un secret technique à un outil non autorisé. Ne
poursuivez pas : supprimez le secret du contexte, révoquez-le si déjà exposé et
signalez l'incident selon la procédure interne.

## Citer une source sans surcharger

Bon format interface :

> Référence : politique interne de classification des données, section « données
confidentielles ».

ou :

> À vérifier dans la source primaire avant réutilisation.

Éviter :

- trois liens dans un feedback court ;
- source non reliée au claim ;
- citation longue ;
- jargon réglementaire sans explication.

## Incertitude

Quand le contexte ne permet pas de conclure :

> Le contexte ne suffit pas pour décider si l'usage est autorisé. La bonne action
est de vérifier la politique interne ou de demander une validation DPO/sécurité.

Ne pas forcer une bonne réponse artificielle.

## Misconceptions

Relier le feedback à la croyance corrigée :

| Misconception | Feedback pattern |
| --- | --- |
| demander source = vérifier | bonne intuition + vérifier source primaire |
| outil interne = sûr | outil autorisé + droits/logs/rétention |
| retirer nom = anonymiser | pseudonymisation vs anonymisation |
| RAG = fiable | source récupérée à vérifier |
| agent = chatbot | permissions/actions à contrôler |

## Anti-patterns

- « Bravo » sans explication.
- « Faux » sans action.
- Feedback plus long que le scénario au niveau débutant.
- Humour sur une erreur utilisateur.
- Certitude juridique non sourcée.
- Conseil de prompt comme solution unique.
- Feedback qui encourage implicitement la dissimulation d'une erreur.

## Templates

### Template court

```text
{Qualification}. {Risque ou bénéfice}. {Action recommandée}.
```

### Template standard

```text
{Qualification}. {Pourquoi ce choix est correct/partiel/risqué dans ce contexte}.
{Action recommandée}. {Source ou limite si utile}.
```

### Template incident/sécurité

```text
À bloquer. {Risque critique}. {Action immédiate}. {Escalade interne}.
```

## Critères d'acceptation

- Chaque feedback explique risque + action.
- Les incertitudes sont visibles.
- Pas de contradiction avec RGPD/sécurité.
- Le ton reste non culpabilisant.
- Le feedback est relié à risques, compétences ou misconceptions.
