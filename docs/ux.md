# UX produit

## Modes

- **Entraînement libre** (défaut) : répétable à volonté, sans enjeu, sans rationnement
  (pas de tickets, pas de vies, pas de chrono). Rejouer une question ou un parcours est
  toujours possible et ne fait jamais régresser le meilleur score affiché
  (cf. `docs/scoring.md`).
- Un futur « mode Évaluation » (parcours figé, restitution organisationnelle) est un
  non-objectif tant qu'aucune ADR ne le cadre — voir `docs/benchmark-jeu-competitif.md`.

## Ton

- Calme, professionnel, pédagogique.
- Pas de gamification humiliante.
- Pas de "piège" gratuit.
- Reconnaître l'incertitude et le contexte.

## Écran d'introduction

Doit préciser :

- objectif : entraînement aux réflexes IA ;
- pas une évaluation RH ;
- durée ;
- données collectées ;
- possibilité de quitter/exporter.

## Écran question

Doit afficher :

- situation ;
- rôle/contexte ;
- action demandée ;
- choix ;
- si média : mention synthétique ou contexte d'analyse ;
- si transcription de conversation (`context.assets`) : présentation type chat,
  la question portant sur l'action prioritaire ;
- bouton "je ne sais pas" acceptable.

## Feedback

Le feedback doit inclure :

- verdict : correct / partiel / risqué / incorrect ;
- raison ;
- risque évité ou créé ;
- action recommandée ;
- source ou fiche réflexe.

Éviter :

- "Bravo !" seul ;
- "Incorrect !" agressif ;
- correction qui humilie ;
- explication dogmatique.

## Synthèse

Doit afficher :

- points forts ;
- axes à renforcer ;
- fiches réflexes recommandées ;
- rappel confidentialité ;
- pas de rang, pas de comparaison nominative.

## Saisie clavier

- Touches `1`-`9` : sélectionner le choix correspondant ; `Entrée` : valider.
- Le clavier est un confort de saisie et un support d'accessibilité, **jamais une prime
  de vitesse** : aucun bonus, aucun malus, aucune mesure de temps de réponse
  (`docs/benchmark-jeu-competitif.md`, Famille D).
- "Je ne sais pas" reste accessible au clavier au même titre que les autres choix.

## Accessibilité

- Navigation clavier complète.
- Contrastes WCAG AA minimum.
- Pas d'information uniquement par couleur.
- Alternatives textuelles pour médias.
- Durées non bloquantes ou ajustables.
- Design mobile-first.

## Médias

Les médias ne doivent jamais être de simples décorations biaisées. Si une image est utilisée, elle doit servir explicitement l'objectif pédagogique.
