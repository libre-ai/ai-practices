# Benchmark — jeu compétitif de référence (jeu compétitif de référence) → Rumble AI Practices

Note de veille et de design. Elle trie ce que `rumble-ai-practices` **prend, adapte ou rejette**
d'un jeu externe d'entraînement aux réflexes IA, et **pourquoi**. Objectif : réutiliser les
emprunts sains, documenter explicitement les non-objectifs, et couper court aux futures
demandes du type « pourquoi pas de ligues / de leaderboard / de chrono ? ».

## Contexte & source

`rumble-ai-practices` est un projet indépendant et **agnostique** : il n'appartient à, ni ne
cible, aucun groupe ou commanditaire tiers.

La source de veille est l'**jeu compétitif de référence**, un jeu propriétaire d'entraînement aux
réflexes IA (plateforme _jeu compétitif de référence_) déployé en entreprise. C'est un
**cas d'école générique** : un jeu compétitif où l'on enchaîne des situations courtes, où l'on
répond vite, où l'erreur coûte une vie, et où le meilleur score classe le joueur dans une ligue
(Starter → Champion) avec multiplicateurs, promotion/relégation et prix nominatifs.

Ce document ne nomme aucun commanditaire, aucune entité ni personne : seul le nom de l'outil
benchmarké (_jeu compétitif de référence / jeu de référence_) est conservé, en tant que source de la comparaison.

### Le même but, une philosophie opposée

Les deux produits visent à **ancrer des réflexes IA**. Ils divergent sur la méthode : l'AI
Challenge est **compétitif et chronométré** ; Rumble est **délibérément non-compétitif**, ce
qui est déjà formalisé dans le repo :

- `scoring.md` — « pas de note globale publique, pas de leaderboard nominatif ».
- `adrs/0003-content-governance-and-no-rh-scoring.md` — le leaderboard nominatif est une
  **alternative rejetée** (« favorise la comparaison sociale plutôt que l'apprentissage »).
- `product-boundaries.md` — le produit « ne possède pas … l'évaluation RH des collaborateurs ».

**Contrainte directrice** : _ne pas pousser à la compétition ni à la course — l'apprentissage
demande du temps et de la pédagogie._ Ce filtre prime sur toute mécanique « vitesse » de la
source.

## Grille de tri

Chaque mécanique de la source est jugée sur trois critères :

1. **Cohérence avec la vision Rumble** — `product-boundaries.md`, `scoring.md`, ADR 0003.
2. **Les 4 axes de décision** — sécurité > qualité > performance > complétude.
3. **La contrainte anti-course** — l'apprentissage demande du temps.

Verdicts possibles : **PRENDRE** · **ADAPTER** · **REJETER** · **DÉJÀ PRÉSENT**.

## Tri par famille

### Famille A — Mécaniques de gameplay

| Élément source                                              | Verdict                                                | Justification                                                                                                                                                                                                                                                                   |
| ----------------------------------------------------------- | ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Partie = suite de situations courtes                        | DÉJÀ PRÉSENT (`crates/session`)                        | Format aligné avec la notion de session ; neutre vis-à-vis de la compétition.                                                                                                                                                                                                   |
| « Répondre le plus vite possible », vitesse → points        | **REJETER**                                            | Contredit « l'apprentissage demande du temps ». La précipitation est dangereuse sur des axes comme `source_verification` (vérifier une source _prend_ du temps) : primer la vitesse enseignerait le mauvais réflexe.                                                            |
| Vies (3 erreurs = fin de partie)                            | **ADAPTER** (non-punitif) — ADR requise                | Rumble refuse la fin-sanction. Seule réinterprétation acceptable : un « garde-fou de charge cognitive » (proposer une pause + remédiation après plusieurs réponses risquées), jamais un _game over_. Trop proche d'une mécanique punitive rejetée → à n'ouvrir qu'avec une ADR. |
| Tickets hebdomadaires (5/sem., expiration le dimanche)      | **REJETER**                                            | Rationner l'entraînement va contre le but pédagogique. C'est un levier d'engagement par rareté, orthogonal à l'apprentissage.                                                                                                                                                   |
| « Meilleur score de la semaine / rejouer ne baisse jamais » | **PRENDRE** (principe)                                 | Principe non-punitif : encourage la répétition sans peur de régresser. Cohérent avec « progression, pas jugement » (ADR 0003). Applicable sans aucune compétition.                                                                                                              |
| Ligues inter-joueurs (Starter → Champion)                   | **REJETER**                                            | Équivaut à un classement relatif entre personnes = leaderboard, explicitement rejeté par ADR 0003 et `scoring.md`.                                                                                                                                                              |
| Paliers de maîtrise individuels                             | DÉJÀ PRÉSENT (`PracticeLevel` : Discovery → Reference) | L'idée de « grimper des niveaux » existe déjà, en version **individuelle et non-comparative**. Rien à importer ; au plus, mieux la valoriser côté UX.                                                                                                                           |
| Multiplicateurs de score ×1–×3, promotion/relégation        | **REJETER**                                            | Récompense de rang et note globale — contraires à `scoring.md` (« pas de note globale publique »).                                                                                                                                                                              |

### Famille B — Matière pédagogique

| Élément source                                            | Verdict                                         | Justification                                                                                                                                                                                    |
| --------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Deux activités : « Training » vs « Compétition »          | **PRENDRE le Training**, laisser la Compétition | Un mode entraînement libre et répétable est exactement l'esprit Rumble. La brique « compétition » est précisément celle qu'on ne suit pas.                                                       |
| « Réflexe » = analyser une situation en quelques secondes | **ADAPTER**                                     | Garder l'intention — ancrer des **réflexes méthodiques** — mais retirer le chrono. « Bon réflexe » ≠ « réponse rapide » : c'est savoir vérifier, contextualiser, refuser.                        |
| Situations réalistes, non triviales                       | DÉJÀ PRÉSENT & supérieur chez Rumble            | Rumble propose des scénarios sourcés, **non binaires**, avec feedback nuancé (bon / risqué / insuffisant / critique) — plus riche qu'un QCM binaire rapide. À revendiquer comme différenciateur. |
| Feedback immédiat après réponse                           | **PRENDRE / confirmer**                         | Rumble a déjà un feedback pédagogique par choix ; la boucle « réponse → explication » est confirmée comme bonne pratique.                                                                        |

### Famille C — Gouvernance & conformité

| Élément source                                                                                        | Verdict                                                            | Justification                                                                                                                                                        |
| ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Règlement structuré (éligibilité, dotations, rétention, litiges)                                      | **PRENDRE comme modèle** (si un « challenge » est un jour déployé) | Bon squelette juridique générique ; à archiver comme référence, pas à implémenter tant qu'aucun événement n'est organisé.                                            |
| RGPD : base légale (intérêt légitime), rétention datée, fonction DPO, droits des personnes, voie CNIL | **PRENDRE** (aligné)                                               | Cohérent avec `security-rgpd.md` ; peut enrichir la checklist de conformité existante. DPO et CNIL sont ici des **fonctions/institutions génériques**, pas des noms. |
| Anti-triche : interdiction d'utiliser une IA pour répondre, sous peine d'exclusion                    | **NOTER** (paradoxe pédagogique)                                   | Un jeu _sur_ l'IA qui interdit l'IA pour y répondre. L'enjeu est moindre pour Rumble (entraînement ≠ concours doté), mais la posture mérite d'être documentée.       |
| Données non nominatives / résultats agrégés                                                           | DÉJÀ ALIGNÉ (Rumble va plus loin)                                  | Rumble impose l'anonymisation par cohorte et des **seuils anti-réidentification**.                                                                                   |

### Famille D — Ergonomie & UI

| Élément source                                   | Verdict                               | Justification                                                                                                                                                                                                             |
| ------------------------------------------------ | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Saisie clavier (touches 1/2/3 + Entrée)          | **PRENDRE** (accessibilité / confort) | Neutre philosophiquement, améliore l'ergonomie sur desktop. **À découpler de tout bonus de vitesse** — dans la source, le clavier sert à « répondre plus vite pour marquer plus » ; on ne garde que le confort de saisie. |
| Souris / tactile / mobile                        | DÉJÀ PRÉVU                            | Rumble est une PWA mobile-first (Dioxus). Rien à importer.                                                                                                                                                                |
| « La vitesse rapporte des points » (via clavier) | **REJETER**                           | Même raison anti-pédagogique que la vitesse notée en Famille A.                                                                                                                                                           |

## Principe directeur

De l'jeu de référence, Rumble retient l'**ossature d'engagement non-punitive** — parties courtes,
meilleur score sans régression possible, feedback immédiat, mode Training libre, confort de
saisie clavier — et le **cadre RGPD / gouvernance**. Rumble **rejette l'intégralité du moteur
compétitif** : vitesse notée, vies-sanction, tickets, ligues, multiplicateurs, leaderboard,
prix nominatifs. Ce moteur contredit à la fois la vision documentée (ADR 0003) et la contrainte
« l'apprentissage demande du temps ».

En une phrase : **on emprunte la forme (situations courtes, boucle de feedback, ergonomie), on
refuse le fond compétitif.**

## Backlog priorisé

Items retenus ou adaptés, à exécuter **ultérieurement** (hors de cette note ; le MVP — un
corpus de questions validées — reste la priorité, on ne code aucune mécanique tant qu'il n'est
pas atteint).

| Prio | Item                                                                                              | Cible                        | Dépendance                                                    |
| ---- | ------------------------------------------------------------------------------------------------- | ---------------------------- | ------------------------------------------------------------- |
| P1   | Formaliser un « mode Entraînement libre » (répétable, sans enjeu) vs un futur « mode Évaluation » | `docs/ux.md`                 | —                                                             |
| P1   | Inscrire le principe « meilleur score / rejouer ne fait jamais régresser »                        | `docs/scoring.md`            | —                                                             |
| P1   | Consigner les non-objectifs importés (ci-dessous) comme frontières produit                        | `docs/product-boundaries.md` | —                                                             |
| P2   | Ergonomie clavier 1/2/3 + Entrée, **découplée de toute prime de vitesse**                         | `docs/ux.md`, `crates/ui`    | —                                                             |
| P2   | Enrichir la checklist conformité (rétention datée, base légale, droits, anti-triche)              | `docs/security-rgpd.md`      | —                                                             |
| P3   | « Garde-fou de charge cognitive » (réinterprétation non-punitive des vies)                        | nouvelle ADR                 | **ADR obligatoire** (proche d'une mécanique punitive rejetée) |

## Non-objectifs importés

Mécaniques de la source que Rumble **n'implémente pas**, par décision de conception (renvoi à
`product-boundaries.md` et ADR 0003) :

- **Vitesse notée / chrono** — prime la précipitation, anti-pédagogique.
- **Vies-sanction / _game over_** — punit l'erreur au lieu de la remédier.
- **Tickets / rationnement hebdomadaire** — restreint l'entraînement.
- **Ligues inter-joueurs, promotion/relégation** — comparaison sociale.
- **Multiplicateurs de score, note globale** — réduit l'apprentissage à un chiffre.
- **Leaderboard nominatif, prix nominatifs** — exposent les personnes, détournent de
  l'apprentissage.

Toute demande future de réintroduire l'une de ces mécaniques doit passer par une nouvelle ADR
(et, si des données personnelles sont en jeu, une analyse d'impact).
