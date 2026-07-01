# Gouvernance de contenu

## Règle d'or

Un contenu non relu est un contenu non publiable.

## Rôles

| Rôle | Responsabilité |
| --- | --- |
| Auteur | rédige scénario, choix, feedback, sources |
| Relecteur métier | vérifie réalisme et vocabulaire |
| Relecteur sécurité/RGPD | vérifie risques données, PII, secrets |
| Relecteur biais | vérifie stéréotypes, représentations, médias |
| Mainteneur corpus | valide le passage `approved` |

## Workflow

```text
draft -> review -> approved
          |          |
          v          v
       blocked    retired
```

## Critères de publication

- Question compréhensible sans deviner l'intention de l'auteur.
- Réponse correcte stable dans le contexte donné.
- Feedback pédagogique et actionnable.
- Sources citées ou justification claire si source interne non publique.
- Pas de contradiction avec sécurité/RGPD.
- Pas de média IA non audité.
- Date de revue et relecteurs renseignés.

## Critères de blocage

- Affirmation factuellement fausse.
- Bonne pratique dangereuse.
- Correction dogmatique sans contexte.
- Biais visuel ou narratif non maîtrisé.
- Incitation à envoyer secrets, PII, logs, code sensible à un outil IA externe.
- Confusion entre demander une source et vérifier une source.
- Usage RH implicite.

## Usage de l'IA pour produire le contenu

Autorisé pour :

- proposer des brouillons ;
- générer des variantes de formulation ;
- lister des risques à vérifier ;
- aider à créer des contre-exemples.

Interdit sans validation humaine :

- publier une question ;
- publier une correction ;
- générer un média humain réaliste ;
- inventer des sources ;
- décider qu'un biais est acceptable.

## Versionnement

Chaque question conserve :

- `id` stable ;
- `version` incrémentée à chaque changement sémantique ;
- historique git ;
- raison du retrait si `retired`.

## Revue périodique

- Questions sécurité/RGPD : tous les 6 mois.
- Questions sources/hallucinations : tous les 6 mois.
- Questions prompt pratique : tous les 12 mois ou à chaque changement doctrinal.
- Médias IA : revue à chaque réutilisation dans un nouveau contexte.
