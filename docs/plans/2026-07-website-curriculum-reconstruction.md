# Plan — reconstruire les apprentissages issus de Website

Date : 2026-07-13  
Statut : infrastructure et trois activités pilotes implémentées en `draft` ; aucune activité n’est promue automatiquement.  
Source : `libre-ai/website/docs/reviews/ai-practices-transfer-brief.md` et son registre de 82 pistes non validées.

## Décision après challenge

Le corpus Website ne devient pas six cours linéaires dans AI Practices.

AI Practices entraîne d’abord des décisions professionnelles. Les concepts techniques sont retenus seulement lorsqu’ils améliorent un comportement observable : vérifier, protéger, cadrer, évaluer, limiter une permission, documenter une décision ou arrêter un système de façon sûre.

Deux pistes sont séparées :

1. **Socle appliqué** — public large et professionnel, situations courtes, feedback sourcé ;
2. **Laboratoires constructeur** — piste facultative pour praticiens techniques, expériences reproductibles, aucun prérequis implicite.

Le laboratoire « modèle minimal » n’est pas un objectif autonome du produit. Il n’est accepté que s’il démontre une pratique mesurable — causalité, déterminisme, contamination, coût ou limite — mieux qu’une expérience plus petite.

## Ce qui existe déjà

Le domaine gouverné couvre déjà :

- sortie IA non probante ;
- données personnelles, secrets et minimisation ;
- vérification des sources et mauvaise attribution ;
- RAG, groundedness et obsolescence ;
- injection indirecte, agents et moindre privilège ;
- biais, médias synthétiques et responsabilité humaine ;
- séparation stricte entre apprentissage et évaluation RH.

Le transfert ne doit donc pas créer de doublons sur hallucinations, RAG, agents ou médias synthétiques. Il doit enrichir les situations et les preuves reliées aux concepts existants.

## Delta pédagogique retenu

| Brief Website | Destination | Verdict |
| --- | --- | --- |
| Représenter et prédire | laboratoire constructeur `model-behaviour` | adapter : tokenisation, distribution et décodage reliés à surconfiance et vérification |
| Entraîner et adapter | laboratoire `evaluation-and-adaptation` | adapter : données, juge, métrique et hypothèses ; pas de recette de fine-tuning sans évaluation |
| Construire un modèle minimal | expérience interne du laboratoire précédent | conditionnel : petit corpus autorisé, dépendances verrouillées, aucune promesse de qualité industrielle |
| Déployer sans masquer les risques | parcours avancé `safe-ai-operation` | prioritaire : permissions, RAG évalué, incident, mode dégradé et arrêt sûr |
| Comprendre la multimodalité | extension de `bias-media-responsibility` | adapter : provenance, erreurs de perception, injection indirecte et accessibilité |
| Fiabilité grand public | enrichissement de `sources-and-reliability` | fusionner : mesurer erreur, omission et citation invalide plutôt que réciter « hallucination » |

## Activités pilotes à construire

Toutes restent `draft` jusqu’à revue humaine et validation de sources primaires.

### A. Une réponse plausible n’est pas une mesure

- **Objectif observable** : choisir un protocole de vérification proportionné à l’enjeu.
- **Situation** : trois réponses avec styles et températures différents sur un corpus connu.
- **Action** : comparer erreurs, omissions et citations invalides.
- **Réussite** : ne pas confondre stabilité, assurance et vérité.
- **Concepts existants** : `concept-ai-output-not-proof`, `concept-overconfidence`, `concept-source-verification`.

### B. Un RAG peut citer sans soutenir

- **Objectif observable** : séparer récupération, attribution et soutien du claim.
- **Situation** : une source pertinente mais obsolète, une citation exacte hors portée et une source primaire valide.
- **Action** : accepter, corriger ou refuser chaque claim.
- **Réussite** : documenter date, portée et passage probant.
- **Concepts existants** : `concept-rag`, `concept-groundedness`, `concept-citation-not-proof`.

### C. L’outil n’obtient pas tous les droits

- **Objectif observable** : réduire les permissions et placer un gate humain avant effet externe.
- **Situation** : un agent doit lire un dossier, produire un brouillon puis envoyer un message.
- **Action** : choisir capacités, durée, journal et rollback.
- **Réussite** : distinguer lecture, écriture et envoi ; refuser la délégation globale.
- **Concepts existants** : `concept-agent`, `concept-least-privilege`, `concept-indirect-prompt-injection`.

### D. Une image est aussi une entrée hostile

- **Objectif observable** : vérifier provenance, accessibilité et instructions indirectes d’un média.
- **Situation** : document multimodal contenant texte visible, métadonnées et instruction incorporée.
- **Action** : isoler, décrire, vérifier et décider si le média peut être traité.
- **Réussite** : ne pas limiter l’analyse au biais visuel ou au caractère synthétique.
- **Concepts existants** : `concept-synthetic-media`, `concept-indirect-prompt-injection`, `concept-bias`.

### E. Amélioration du score ou déplacement du juge

- **Objectif observable** : contester une conclusion d’amélioration lorsque données, métrique ou évaluateur ont changé.
- **Situation** : comparaison avant/après adaptation avec jeu de test et juge différents.
- **Action** : identifier les variables empêchant la conclusion.
- **Réussite** : demander protocole stable, sorties brutes et limites d’extrapolation.
- **Nouveaux concepts candidats** : `concept-evaluation-protocol`, `concept-judge-bias`, `concept-test-contamination`.

### F. Arrêter proprement vaut mieux que répondre à tout prix

- **Objectif observable** : choisir refus, mode dégradé ou arrêt sûr selon le risque.
- **Situation** : source indisponible, fournisseur en erreur et journal incomplet pendant une action sensible.
- **Action** : sélectionner le comportement opérationnel.
- **Réussite** : préserver données et décision humaine sans fallback opaque.
- **Nouveaux concepts candidats** : `concept-safe-stop`, `concept-degraded-mode`, `concept-provider-failure`.

## Contrat à ajouter avant les laboratoires techniques

Le schéma `question.schema.json` couvre les situations à choix, pas une expérience reproductible. Ne pas forcer les laboratoires dans ce modèle.

Définir d’abord un contrat `activity.v1` avec :

- identifiant, version, statut et locale ;
- objectif observable et prérequis ;
- type `scenario`, `experiment` ou `simulation` ;
- environnement et dépendances verrouillés pour une expérience ;
- données autorisées et licence ;
- consigne, sorties attendues et critères de réussite ;
- feedback explicatif ;
- risques, limites et condition d’arrêt ;
- claims reliés aux sources ;
- auteur humain, reviewers, assistance IA et date de revue ;
- provenance et texte alternatif de chaque média ;
- condition de retrait.

## Séquence d’exécution

1. ⏳ Valider humainement le découpage socle/laboratoires.
2. ✅ Créer le schéma `activity.v1`, son domaine Rust, son validateur CLI et ses tests négatifs.
3. ⏳ Réouvrir les pistes de sources ; les sources actuelles restent `draft` et doivent être revues claim par claim.
4. ✅ Construire A, B et C sur les concepts existants, en `draft` avec assistance IA déclarée.
5. ⏳ Construire D avec le gate média existant.
6. Ajouter les concepts candidats nécessaires à E et F, avec claims et sources.
7. Prouver une activité de chaque type dans le moteur de session.
8. Promouvoir une activité seulement après revue humaine métier, sécurité et éditoriale.

## Gates

```bash
./scripts/test-postgres-disposable.sh
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- validate-activities --activities content/activities
cargo run -p rumble-ai-practices-cli -- audit-corpus \
  --content content/questions --media content/media --out reports/audit.json
```

Le contrat couvre désormais les fixtures d’activité, les sources obligatoires, l’environnement verrouillé des expériences et la revue humaine avant `approved`. Restent à ajouter avec D : le refus d’un média sans provenance et le retrait automatique d’une activité expirée.

## Non-objectifs

- copier prose, chiffres, images ou structure MDX du Website ;
- publier les 82 URL comme bibliographie validée ;
- promettre un cursus complet de ML ou de construction de LLM ;
- transformer une expérience technique en certification ;
- promouvoir automatiquement un contenu généré ou préparé par un agent.
