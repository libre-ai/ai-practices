---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../ux.md
  - ./pedagogy.md
  - ./analytics-privacy.md
  - ../adrs/0003-content-governance-and-no-rh-scoring.md
---

# Principes UX pédagogiques

## Objectif

Aligner expérience utilisateur, accessibilité, respect et apprentissage.

## Principes

- Calme avant compétition.
- Feedback avant score.
- Synthèse privée avant comparaison.
- Accessibilité native, pas ajout tardif.
- Transparence sur données.
- Aucun dark pattern.

## Écran d'introduction

Doit préciser :

- objectif pédagogique ;
- durée estimée ;
- données collectées ;
- absence d'évaluation RH ;
- possibilité d'arrêter ;
- mode offline/local si disponible.

## Écran question

Doit afficher :

- rôle/contexte ;
- situation ;
- action demandée ;
- choix ;
- option « je ne sais pas » si utile ;
- média seulement si nécessaire ;
- mention synthétique si média IA.

## Feedback

Doit afficher :

- verdict accessible ;
- risque créé/évité ;
- action recommandée ;
- source ou fiche réflexe ;
- réponse partielle reconnue.

## Synthèse

Doit afficher :

- axes forts ;
- axes à renforcer ;
- recommandations ;
- confidentialité ;
- distribution privée seulement si k-anonymat respecté.

Ne doit pas afficher :

- rang ;
- leaderboard ;
- note globale humiliante ;
- comparaison à un collègue ;
- vitesse.

## Accessibilité

- navigation clavier complète ;
- contrastes WCAG AA minimum ;
- pas d'information uniquement par couleur ;
- alternatives textuelles pour médias ;
- taille tactile mobile ;
- focus visible ;
- feedback lisible par lecteur d'écran ;
- temps non bloquant.

## Privacy UX

- expliquer les données collectées en langage clair ;
- distinguer local/anonyme/authentifié ;
- indiquer si une distribution est indisponible sous seuil ;
- permettre suppression locale ;
- éviter jargon RGPD dans l'interface principale.

## Anti-patterns

- chrono ;
- vies ;
- tickets ;
- streak culpabilisant ;
- popups de rétention ;
- « vous êtes moins bon que » ;
- couleur rouge seule pour erreur ;
- média décoratif biaisé.

## Critères d'acceptation

- Pas de leaderboard.
- Feedback accessible.
- Synthèse privée.
- Contrastes/tokens respectés.
- L'UX ne pousse pas à exposer de données.
