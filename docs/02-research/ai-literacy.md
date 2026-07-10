---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./bibliography.md
  - ../03-domain-model/ai-literacy-framework.md
  - ../03-domain-model/competency-model.md
---

# AI literacy

## Objectif

Définir ce que signifie être compétent face à l'IA dans un contexte
professionnel. La littératie IA visée par `rumble-ai-practices` n'est pas une
culture générale : c'est une capacité à agir correctement sous contraintes.

## Définition de travail

La littératie IA professionnelle combine :

1. compréhension minimale des capacités et limites ;
2. reconnaissance des risques de données, sources, biais, sécurité et
   responsabilité ;
3. capacité à formuler une tâche sans exposer d'informations sensibles ;
4. vérification et documentation avant réutilisation ;
5. décision de ne pas utiliser l'IA quand le contexte est trop risqué.

Source candidate principale : `source-long-magerko-ai-literacy-2020`, à compléter
par une revue récente AI literacy.

## Dimensions

| Dimension | Question utilisateur | Exemple observable |
| --- | --- | --- |
| Conceptuelle | Qu'est-ce que le système peut ou ne peut pas faire ? | sait qu'une sortie plausible peut être fausse |
| Données | Quelles informations sont exposées ? | retire PII/secrets avant prompt |
| Fiabilité | Comment vérifier ? | remonte à une source primaire |
| Sécurité | Quel abus est possible ? | identifie prompt injection indirecte |
| Éthique/biais | Qui peut être lésé ? | détecte stéréotype ou proxy discriminant |
| Responsabilité | Qui décide et assume ? | garde validation humaine sur décision à impact |
| Organisation | Quelle règle interne s'applique ? | consulte politique d'outil autorisé |
| Métacognition | Qu'est-ce que j'ignore ? | signale incertitude au lieu d'inventer |

## Niveaux de maîtrise

| Niveau | Description | Produit |
| --- | --- | --- |
| `awareness` | reconnaît une notion ou un risque | feedback explicatif simple |
| `guided` | applique une règle dans un cas clair | QCM contextualisé |
| `autonomous` | arbitre dans un cas ambigu | scénario multi-risques |
| `mentor` | explique et aide autrui | fiche réflexe, justification argumentée |

## Compétences minimales

- Protéger les données personnelles, secrets et documents internes.
- Choisir un outil autorisé selon la sensibilité.
- Formuler une demande utile sans sur-exposer.
- Vérifier une affirmation et une source.
- Reconnaître hallucination, citation inventée et mauvaise attribution.
- Identifier un biais ou une représentation problématique.
- Évaluer le risque d'un agent ou connecteur.
- Maintenir la responsabilité humaine.
- Documenter les limites et hypothèses.
- Escalader vers DPO, sécurité, juridique ou métier.

## Ce que la littératie IA n'est pas

- Savoir écrire des prompts sophistiqués.
- Connaître tous les détails mathématiques des LLM.
- Être capable de choisir le « meilleur » modèle.
- Obtenir un score supérieur à ses collègues.
- Remplacer une formation sécurité, RGPD ou métier.

## Traduction en scénarios

| Compétence | Scénario type | Risque principal |
| --- | --- | --- |
| protéger les données | synthèse d'un contrat client | confidentialité / RGPD |
| vérifier | citation juridique générée | source verification |
| cadrer | demande de synthèse projet | secret métier |
| détecter biais | génération d'image recrutement | biais / média synthétique |
| encadrer agent | agent qui envoie des emails | sécurité / responsabilité |
| documenter | note stratégique assistée par IA | traçabilité / incertitude |

## Implications pour le modèle de compétences

Le futur `ai-literacy-framework.md` doit :

- séparer connaissance, action, jugement et responsabilité ;
- éviter un score global ;
- proposer des niveaux observables ;
- relier chaque compétence à des risques et concepts ;
- permettre une remédiation par axe.

## Limites

- La littérature AI literacy est hétérogène : certains cadres sont scolaires,
  d'autres citoyens, d'autres professionnels.
- Les compétences nécessaires varient selon secteur, outils et politiques
  internes.
- Un questionnaire ne prouve pas seul le transfert en situation réelle.

## Critères d'acceptation

- Le document alimente directement `ai-literacy-framework.md`.
- Les compétences sont observables par choix/scénario.
- Les limites du modèle sont explicites.
- Aucun niveau ne crée une certification RH implicite.
