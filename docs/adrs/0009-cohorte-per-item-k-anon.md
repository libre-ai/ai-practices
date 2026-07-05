# ADR 0009 — Cohorte per-item k-anonyme (extension d'ADR 0006)

## Statut

Acceptée. **Étend ADR 0006** (cohorte k-anonyme) de la granularité **par-axe** à la granularité **par-item**. Hérite de tous les invariants de 0006.

## Contexte

ADR 0006 définit une cohorte anonyme k-anonyme **par axe pédagogique** (distribution + position, servie si le groupe compte ≥ k sessions). Le jeu de sensibilisation (ADR 0007) a besoin d'un feedback **par item** — « X % ont jugé cette image non biaisée » — affiché **à chaque choix**, pour montrer que le piège fonctionne sur la majorité. C'est le cœur pédagogique : **déculpabiliser** (le problème n'est pas l'individu, puisque tout le monde tombe dans le piège).

## Décision

- Extension du modèle 0006 à une distribution **par item** : table `item_answers(item_id, choice_id, count)` + agrégation, avec les **mêmes invariants** que 0006 (anonyme, k-anonyme, rétention, zéro PII, bounded context, scoring serveur).
- **Masquage sous k** : la distribution d'un item n'est servie que si ≥ k réponses (défaut **k = 5**, réutilisé de 0006) ; en-dessous, l'API répond « indisponible » — jamais d'agrégat partiel.
- **Positionnement de fin de session** en **quartiles + courbe de distribution**, via la cohorte **par-axe** existante (0006).
- **Non-compétitif** : agrégat solidaire (« tu n'es pas seul »), **ni streak, ni classement nominatif** (ADR 0003 conservé sur ce point).

## Règles

- Le schéma per-item ne stocke que `(item_id, choice_id, count)` **agrégé** — aucun champ nominatif, aucune trace individuelle.
- Le seuil `k` et la rétention sont **testés** (tests d'intégration, rouge→vert), comme 0006.
- Idempotence par `client_id` opaque : une réponse compte une seule fois (une reprise réseau ne double-compte pas).
- **Dégradation offline gracieuse** : sans backend joignable, la synthèse locale s'affiche seule — le feedback crowd est un bonus, jamais un bloquant.
- Aucune table/route ne traverse vers un autre portail (`*_ADDON_URI` distinct ; garde-fou CI, comme 0006).

## Conséquences

Positives :

- le feedback social par item **sert directement le manifeste** (le biais trompe la majorité → la faute n'est pas individuelle) ;
- réutilise le pattern 0006 (store, distribution, seuil k, rétention) — coût marginal.

Coûts :

- un schéma + un endpoint de plus ;
- volume de données par item (borné par la rétention).

## Alternatives rejetées

- **Pourcentages seed statiques** (inventés par item) : immédiat mais **faux** ; trahit la promesse d'honnêteté du produit.
- **Cohorte par-axe seule (0006)** : ne fournit pas le « par image, à chaque choix » recherché.
- **Best-score client-only (drill-formats I7)** : métrique personnelle de progression, pas le feedback social solidaire visé.
