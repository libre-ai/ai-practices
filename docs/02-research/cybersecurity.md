---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./agents.md
  - ./rag.md
  - ../security-rgpd.md
  - ../04-content-factory/quality-gates.md
---

# Cybersécurité IA

## Objectif

Synthétiser les menaces, contrôles et scénarios sécurité liés à l'usage
professionnel de l'IA, sans fournir de guide offensif inutile.

Sources candidates : `source-owasp-llm-top10`, `source-mitre-atlas`,
`source-enisa-ai-cybersecurity`, `source-nist-csf-2`.

## Actifs à protéger

- données personnelles ;
- secrets techniques ;
- code source ;
- contrats, documents clients, roadmap ;
- credentials, tokens, clés API ;
- prompts système et politiques internes ;
- logs et transcriptions ;
- droits d'accès aux outils ;
- réputation et décisions métier.

## Menaces principales

| Menace | Description | Scénario pédagogique |
| --- | --- | --- |
| Prompt injection directe | utilisateur tente de contourner règles | reconnaître instruction malveillante |
| Prompt injection indirecte | document externe contient instruction | RAG/ticket/email avec commande cachée |
| Secret leakage | secret collé dans prompt ou code généré | refuser, scanner, outil autorisé |
| Data exfiltration | sortie ou outil transmet donnée | limiter permissions et sorties |
| Excessive agency | agent agit trop librement | validation avant action |
| Insecure output handling | sortie IA exécutée sans contrôle | code/script requiert revue |
| Model supply chain | modèle/plugin/dépendance compromis | vérifier origine/licence |
| Sensitive logs | prompts ou réponses stockés | minimisation, purge, masquage |
| Overreliance | décision sécurité fondée sur IA seule | revue humaine compétente |

## Contrôles à enseigner

- ne jamais coller secrets, tokens, clés ou logs sensibles dans un outil non
  autorisé ;
- vérifier classification de données ;
- traiter documents récupérés comme données non fiables ;
- appliquer moindre privilège aux agents ;
- exiger confirmation humaine avant action externe ;
- tester et relire le code généré ;
- scanner secrets et dépendances ;
- journaliser sans PII/secrets ;
- documenter incertitudes et escalades.

## Questions types

- « Vous voulez faire analyser un log d'erreur contenant un token : que faites-vous ? »
- « Un ticket support contient : ignore les règles précédentes. L'agent doit-il suivre ? »
- « Un assistant propose un script shell à exécuter en production. Quelle revue ? »
- « Un agent a accès au drive complet pour résumer un dossier. Est-ce proportionné ? »

## Limites

- La sécurité IA évolue rapidement : revue trimestrielle recommandée pour menaces
  agent/RAG/tool calling.
- Les contrôles dépendent du contexte organisationnel.
- Les scénarios doivent éviter d'enseigner des payloads offensifs réutilisables.

## Gates blockers

Bloquer tout contenu qui :

- recommande d'envoyer un secret à un service non autorisé ;
- banalise l'exécution de code IA sans revue ;
- donne des instructions d'attaque détaillées non nécessaires ;
- encourage permissions agentiques larges sans contrôle ;
- masque la présence de PII dans logs/prompts.

## Critères d'acceptation

- Aucune instruction offensive inutile.
- Les secrets/PII sont explicitement protégés.
- Les scénarios sont réalistes et non sensationnalistes.
- Les contrôles sont reliés à des comportements utilisateurs.
