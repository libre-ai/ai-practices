# Grille d'audit de l'existant

## Objectif

Auditer l'application actuelle avant de la remplacer, pour identifier ce qui doit être supprimé, réécrit ou conservé.

## Fiche d'audit

```yaml
id: audit-001
source: capture|export|video|question
original_text: "..."
screenshot: path/to/file.png
category:
  - factual_error
  - misleading_feedback
  - oversimplification
  - prompt_dogma
  - privacy_risk
  - security_risk
  - media_bias
  - missing_source
  - ambiguous_question
severity: low|medium|high|critical
problem: "Pourquoi c'est problématique"
recommendation: remove|rewrite|contextualize|source|block
replacement_idea: "Proposition de scénario corrigé"
reviewer: TBD
status: open|accepted|fixed|rejected
```

## Catégories clés

### Erreur factuelle

Exemple : "demander des liens de sources à l'IA suffit à vérifier".

Correction attendue : demander des sources est une étape, mais il faut vérifier existence, contenu, date et autorité.

### Simplification abusive

Exemple : "un brouillon destiné à publication ne contient pas de données sensibles".

Correction attendue : vérifier contenu réel, commentaires, métadonnées, classification.

### Dogme de prompt

Exemple : présenter une mnémonique comme méthode universelle.

Correction attendue : distinguer astuce de formulation et exigence de sécurité/qualité.

### Biais média

Exemple : image IA caricaturale pour illustrer un biais sans analyser ses propres stéréotypes.

Correction attendue : grille de biais et décision documentée.

## Sortie attendue

Un rapport `reports/audit-existing.json` ou Markdown listant :

- nombre d'items audités ;
- nombre de blocages critiques ;
- thèmes à réécrire ;
- exemples à conserver ;
- décisions éditoriales.
