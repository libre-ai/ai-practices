---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./bibliography.md
  - ../05-product-learning/pedagogy.md
  - ../04-content-factory/feedback-writing-guide.md
---

# Sciences cognitives et apprentissage

## Objectif

Identifier les principes d'apprentissage utiles pour concevoir des sessions
efficaces, respectueuses et transférables en contexte professionnel.

## Principes robustes pour le produit

| Principe | Idée utile | Source candidate | Implication produit |
| --- | --- | --- | --- |
| Retrieval practice | récupérer activement une connaissance renforce l'apprentissage | `source-roediger-karpicke-2006` | faire appliquer une règle dans plusieurs scénarios |
| Spacing | espacer les rappels améliore la rétention | `source-cepeda-spacing-2006` | recommander retours par axe plutôt qu'une session unique |
| Feedback | un feedback utile répond à où j'en suis, où aller, comment | `source-hattie-timperley-feedback-2007` | chaque correction doit expliquer risque + action |
| Charge cognitive | trop d'informations simultanées nuit à l'apprentissage | `source-sweller-cognitive-load` | calibrer longueur et complexité par niveau |
| Multimédia | images et texte doivent réduire, pas augmenter, la charge | `source-mayer-multimedia-learning` | médias seulement si pédagogiquement utiles et audités |
| Objectifs cognitifs | comprendre/appliquer/analyser ne sont pas équivalents | `source-bloom-taxonomy` | varier reconnaissance, application et arbitrage |
| Métacognition | savoir ce qu'on ignore améliore le jugement | source à compléter | inclure réponses d'incertitude et d'escalade |

## Application à `rumble-ai-practices`

### Débutant

- scénarios courts ;
- un risque principal ;
- feedback direct ;
- vocabulaire défini ;
- action simple : vérifier, anonymiser, refuser, demander politique.

### Intermédiaire

- plusieurs contraintes ;
- réponses partielles ;
- conflit entre productivité et sécurité ;
- choix d'outil ou source ;
- feedback comparatif.

### Avancé

- scénarios ambigus ;
- agent/RAG/média ;
- politique interne ;
- arbitrage documenté ;
- reconnaissance de ce qui ne peut pas être conclu.

## Anti-patterns cognitifs

- **Illusion de fluidité** : une sortie claire semble vraie.
- **Illusion de compétence** : réussir un quiz simple donne une confiance
  excessive.
- **Surcharge** : trop de concepts IA dans une question débutant.
- **Feedback pauvre** : indiquer seulement la bonne réponse.
- **Transfert faible** : cas trop abstrait pour être reconnu au travail.
- **Gamification distractive** : chrono et leaderboard déplacent l'attention vers
  la performance sociale.

## Règles éditoriales dérivées

1. Un scénario débutant ne doit entraîner qu'un risque principal.
2. Un feedback doit expliquer pourquoi le choix crée ou réduit un risque.
3. Les concepts doivent réapparaître dans plusieurs contextes.
4. Les médias doivent être utiles à la décision, pas décoratifs.
5. Une réponse « je ne peux pas conclure » doit être possible quand le contexte
   manque.
6. La synthèse doit proposer une prochaine action, pas une étiquette de niveau
   humiliant.

## Controverses / prudence

- Les effets pédagogiques ne se transfèrent pas automatiquement à tout public.
- Les taxonomies d'objectifs sont utiles pour structurer, mais ne garantissent
  pas la qualité.
- Les principes neuroscientifiques populaires doivent être évités s'ils ne sont
  pas soutenus par des méta-analyses.

## Critères pour scénarios et feedbacks

- Le niveau de difficulté est justifié par nombre de risques, ambiguïté et
  concepts requis.
- Le feedback donne une action transférable.
- Les distracteurs représentent des erreurs plausibles, pas des pièges.
- La répétition varie les contextes pour éviter l'apprentissage mécanique.

## Critères d'acceptation

- Chaque principe devient une règle éditoriale ou UX.
- Les controverses sont signalées.
- Aucune affirmation neuroscientifique faible n'est utilisée comme preuve.
- Le document alimente `pedagogy.md` et les guides de feedback.
