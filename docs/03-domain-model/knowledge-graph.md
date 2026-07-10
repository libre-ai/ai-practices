---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ./taxonomy.md
  - ./ontology.md
  - ./competency-model.md
  - ./risk-model.md
  - ../06-technical-contracts/provenance.md
---

# Graphe de connaissances

## Objectif

Définir comment représenter les relations entre concepts, sources, claims,
risques, compétences et contenus.

Le graphe sert à :

- produire des contenus cohérents ;
- détecter concepts orphelins ou sources faibles ;
- construire parcours et remédiations ;
- expliquer pourquoi une question existe ;
- auditer la couverture du corpus.

## Non-objectifs

- Choisir une base graphe définitive.
- Construire un moteur RAG générique.
- Produire une visualisation avancée.
- Remplacer la revue humaine par un score de graphe.

## Noeuds

| Type | ID prefix | Description |
| --- | --- | --- |
| `Source` | `source-` | référence documentaire |
| `Claim` | `claim-` | affirmation vérifiable |
| `Concept` | `concept-` | notion enseignable |
| `Risk` | `risk-` | risque à reconnaître/mitiger |
| `Competency` | `comp-` | compétence observable |
| `LearningObjective` | `lo-` | objectif pédagogique |
| `Misconception` | `misconception-` | croyance erronée |
| `Question` | `q-` | contenu d'entraînement |
| `MediaAsset` | `media-` | média utilisé |
| `ReviewEvent` | `review-` | preuve de revue |
| `LearningPath` | `path-` | parcours/remédiation |

## Arêtes

| Relation | Source -> Cible | Sens |
| --- | --- | --- |
| `evidences` | Source -> Claim | la source soutient le claim |
| `has_claim` | Concept/Question -> Claim | l'objet utilise le claim |
| `requires` | Concept -> Concept | prérequis |
| `clarifies` | Concept -> Concept | désambiguïsation |
| `contrasts_with` | Concept -> Concept | distinction |
| `creates_risk` | Concept/Choice -> Risk | augmente le risque |
| `mitigates` | Concept/Choice -> Risk | réduit le risque |
| `supports_competency` | Concept -> Competency | contribue à compétence |
| `assesses` | Question -> Competency | évalue une compétence |
| `targets` | LearningObjective -> Competency | objectif cible compétence |
| `covers` | Question -> Concept/Risk | contenu couvre concept/risque |
| `corrects` | Feedback/Question -> Misconception | corrige croyance erronée |
| `reviewed_by` | Artifact -> ReviewEvent | preuve de revue |
| `uses_media` | Question -> MediaAsset | média utilisé |
| `remediates` | LearningPath -> Risk/Competency | parcours recommandé |

## Contraintes de graphe

### Contraintes bloquantes

- Pas de cycle dans `requires`.
- Pas de `Question approved` sans `reviewed_by` humain.
- Pas de `Claim high_confidence` sans `evidences`.
- Pas de `Concept approved` sans lien vers `Risk` ou `Competency`.
- Pas de `MediaAsset synthetic_human` sans `reviewed_by` biais/média.
- Pas de `Question approved` si elle couvre un risque blocker non mitigé.

### Contraintes d'avertissement

- Concept sans question associée.
- Risque sans mitigation.
- Compétence sans question de niveau `guided`.
- Source vendeur utilisée seule.
- Claim non revu depuis plus que sa période de fraîcheur.
- Domaine taxonomique sous-couvert.

## Format exportable proposé

```yaml
nodes:
  - id: concept-source-verification
    type: Concept
    label: Vérification des sources
    status: review
  - id: risk-hallucinated-source
    type: Risk
    axis: source_verification
  - id: comp-check-source
    type: Competency
    dimension: verify
edges:
  - from: concept-source-verification
    type: mitigates
    to: risk-hallucinated-source
  - from: concept-source-verification
    type: supports_competency
    to: comp-check-source
```

## Exemple de sous-graphe

```text
source-nist-ai-rmf-1-0
  evidences -> claim-ai-output-needs-verification
    has_claim <- concept-ai-output-not-proof
      requires <- concept-source-verification
        mitigates -> risk-hallucinated-source
        supports_competency -> comp-check-source
          assessed_by <- q-source-001
```

## Usages produit

### Génération de questions

Le graphe permet de demander :

- concept cible ;
- compétence cible ;
- risque principal ;
- misconception à corriger ;
- source/claim à utiliser ;
- prérequis déjà couverts.

### Remédiation

Après une session, les erreurs par axe peuvent pointer vers :

- concepts mal maîtrisés ;
- misconceptions probables ;
- questions de remédiation ;
- fiches réflexes.

### Audit corpus

Rapports possibles :

- concepts sans source ;
- compétences sans questions ;
- risques sans mitigation ;
- questions sans objectif pédagogique ;
- sources expirées ;
- dépendance excessive à sources vendeurs.

## Validation locale future

Commandes candidates à implémenter plus tard :

```bash
cargo run -p rumble-ai-practices-cli -- validate-graph --graph content/graph.yml
cargo run -p rumble-ai-practices-cli -- audit-coverage --content content/questions --graph content/graph.yml
```

## Critères d'acceptation

- Les relations ont un sens non ambigu.
- Les cycles de prérequis sont détectables.
- Le graphe ne contient pas de source inventée.
- Les contraintes bloquantes protègent publication, sources, médias et RH.
- Le format proposé reste simple à convertir en JSON/YAML.
