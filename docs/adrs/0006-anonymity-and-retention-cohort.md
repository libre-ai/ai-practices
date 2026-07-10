# ADR 0006 — Modèle d'anonymat et de rétention (distribution de cohorte k-anonyme)

## Statut

Acceptée. **Opérationnalise** l'exigence « seuils anti-réidentification » d'**ADR 0003**.

## Contexte

ADR 0003 autorise l'affichage d'un **positionnement relatif privé** dans une distribution anonymisée, par axe pédagogique, « si les seuils anti-réidentification sont respectés ». Cette feature n'existe pas encore : `crates/session` renvoie `private_distribution: None`. Elle exige un backend (persistance multi-session + agrégation).

Un produit **sur** la protection des données doit être irréprochable sur les siennes (axe #1, Sécurité). Le modèle d'anonymat se conçoit **avant** toute ligne de backend, pas après.

## Décision

Le backend de cohorte est **anonyme par conception** :

- **Session anonyme** : identifiant opaque généré serveur, **aucun** champ nominatif (ni e-mail, ni nom, ni identifiant réidentifiant). Transport via cookie **HttpOnly** ; pas de compte.
- **k-anonymat** : une distribution par axe n'est servie que si le groupe compte **≥ k sessions** (seuil documenté, défaut **k = 5**) ; en-dessous, l'API répond « indisponible » — jamais d'agrégat partiel.
- **Rétention** : purge automatique des sessions anonymes après un délai documenté (défaut **90 jours**), appliquée par tâche planifiée + migration.
- **Audit** : journal des accès à l'API de cohorte (événements d'accès uniquement, **jamais** de données utilisateur).
- **Zéro PII** en logs (application, Postgres, accès). **Chiffrement-at-rest** prêt (activé au déploiement réel).
- **Bounded context** : une base par produit (`ai-practices` a la sienne) ; aucune table ni transaction ne traverse vers un autre portail (`*_ADDON_URI` distincts).

## Règles

- Le schéma ne stocke que `(session_id opaque, scores_par_axe, horodatage_complétion)` — aucun champ nominatif.
- Le seuil `k` et le délai de rétention sont **testés** (tests d'intégration), pas seulement documentés.
- La suppression d'une session **ne casse pas** l'anonymat de la cohorte : test obligatoire (« un utilisateur rejoint, un autre supprime, les statistiques restent k-anonymes et cohérentes »).
- Le scoring reste **côté serveur** (ADR 0003) ; l'UI n'affiche que le positionnement fourni, elle ne calcule rien.
- Aucune migration/route de ce produit ne référence une table ou un `*_ADDON_URI` d'un autre portail (garde-fou CI dédié).

## Conséquences

Positives :

- capacité d'**anonymisation réutilisable** par tous les `rumble-*` (session anonyme + agrégat k-anonyme + rétention) ;
- la feature de distribution devient **livrable et honnête** (la promesse d'ADR 0003 cesse d'être creuse) ;
- crédibilité RGPD exemplaire — le produit pratique ce qu'il enseigne.

Coûts :

- contrainte de conception forte sur le schéma DB et les contrats d'API ;
- complexité des tests d'anonymat sous suppression et sous seuil.

## Alternatives rejetées

- **Retirer la feature cohorte** : renonce à une promesse cœur (ADR 0003) et à une capacité réutilisable — honnête mais appauvrissant.
- **Pseudonymat avec compte** : viole le « pas de nominatif » et rouvre la porte au classement RH implicite (ADR 0003).
- **Déployer d'abord, anonymiser ensuite** : responsabilité RGPD inacceptable, a fortiori pour un produit sur la protection des données.
