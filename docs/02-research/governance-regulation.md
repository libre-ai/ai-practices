---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
legal_review_status: accepted_for_documentation_baseline_2026-07-05
related:
  - ./bibliography.md
  - ../security-rgpd.md
  - ../adrs/0006-anonymity-and-retention-cohort.md
---

# Gouvernance et régulation IA

## Objectif

Documenter les cadres de gouvernance nécessaires pour produire des scénarios
fiables sans faire de conseil juridique définitif.

## Avertissement

Ce document est une synthèse pédagogique. Il ne constitue pas un avis juridique.
Les obligations doivent être vérifiées par DPO/juridique selon secteur, pays,
rôle de l'organisation, fournisseur, données et usage.

## Cadres à suivre

| Cadre | Source candidate | Apport produit |
| --- | --- | --- |
| RGPD | `source-eu-gdpr-2016-679`, `source-cnil-ai` | données personnelles, minimisation, finalité, droits |
| AI Act | `source-eu-ai-act-2024-1689` | obligations selon rôles et niveaux de risque |
| NIST AI RMF | `source-nist-ai-rmf-1-0` | cartographie et gestion des risques IA |
| ISO/IEC 42001 | `source-iso-iec-42001-2023` | système de management IA |
| ISO/IEC 23894 | `source-iso-iec-23894-2023` | gestion des risques IA |
| DORA | `source-dora-eu-2022-2554` | résilience numérique secteur financier |
| OCDE/UNESCO | `source-oecd-ai-principles`, `source-unesco-ai-ethics` | principes éthiques et sociétaux |
| Politiques internes | source F | règles d'outils, classification, revue |

## Principes transverses

- finalité claire ;
- minimisation des données ;
- transparence ;
- responsabilité humaine ;
- gestion des risques ;
- documentation ;
- contrôle des fournisseurs ;
- sécurité by design ;
- revue périodique ;
- proportionnalité au niveau de risque.

## Impacts pédagogiques

| Sujet | Ce que l'utilisateur doit apprendre |
| --- | --- |
| Données personnelles | identifier, minimiser, demander base/politique si doute |
| Outils autorisés | vérifier politique interne avant usage de données sensibles |
| Décision à impact | garder supervision et documentation humaine |
| Fournisseur externe | considérer rétention, localisation, usage des données, contrat |
| Média synthétique | provenance, consentement, risque de tromperie |
| Agents | droits, logs, audit, contrôle humain |

## Formulations prudentes

Préférer :

- « selon la politique interne » ;
- « à vérifier avec le DPO/juridique » ;
- « peut nécessiter » ;
- « dans un contexte à enjeu » ;
- « la règle dépend du rôle et de l'usage ».

Éviter :

- « toujours légal » ;
- « conforme RGPD » sans analyse ;
- « anonymisé » si seulement pseudonymisé ;
- « certifié AI Act » sans base ;
- « outil interne donc sans risque ».

## Points nécessitant revue juridique/DPO

- Qualification de données personnelles ou sensibles.
- Base légale et finalité.
- Durée de conservation.
- Transfert hors UE.
- Usage de données pour entraîner/améliorer un fournisseur.
- Décision automatisée ou assistance à décision à impact.
- Statut de système à haut risque sous AI Act.
- DPIA/AIPD.

## Scénarios pédagogiques

- Un collaborateur veut pseudonymiser un fichier avant usage IA : est-ce suffisant ?
- Une équipe veut utiliser un outil externe gratuit pour analyser des tickets.
- Un manager demande les scores individuels de formation.
- Un agent IA traite des demandes clients et propose des décisions.
- Une image générée illustre un groupe de personnes dans une campagne interne.

## Critères d'acceptation

- Les formulations restent prudentes.
- Les obligations sont sourcées ou marquées à vérifier.
- Les scénarios distinguent règle générale et politique interne.
- Une revue DPO/juridique est requise avant `stable`.
