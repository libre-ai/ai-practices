---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./cybersecurity.md
  - ./rag.md
  - ../03-domain-model/risk-model.md
---

# Agents IA et tool calling

## Objectif

Documenter les architectures agentiques et leurs risques pour les scénarios
professionnels.

## Définition de travail

Un agent IA est un système qui utilise un modèle pour planifier, décider ou
enchaîner des actions via outils, mémoire ou connecteurs. Le risque augmente
quand le système peut produire des effets hors de la conversation : lire,
écrire, envoyer, supprimer, acheter, publier, modifier.

Sources candidates : `source-owasp-llm-top10`, `source-mitre-atlas`, sources
techniques MCP/tool calling à vérifier.

## Assistant vs agent

| Dimension | Assistant conversationnel | Agent / tool calling |
| --- | --- | --- |
| Sortie principale | texte ou média | texte + actions/outils |
| Risque dominant | erreur de contenu | erreur de contenu + effet externe |
| Contrôle | utilisateur lit la réponse | permissions et validation nécessaires |
| Surface d'attaque | prompt utilisateur | prompts + documents + outils + mémoire |
| Preuve | transcript | logs d'actions, audit, rollback |

## Composants à comprendre

- modèle ;
- orchestrateur ;
- outils ;
- permissions ;
- mémoire ;
- sources documentaires ;
- logs et audit ;
- policy engine éventuel ;
- humain valideur.

## Risques spécifiques

| Risque | Exemple | Contrôle attendu |
| --- | --- | --- |
| Permission excessive | agent email avec envoi libre | moindre privilège, confirmation humaine |
| Injection indirecte | ticket contient instruction malveillante | traiter contenu comme donnée, pas instruction |
| Exfiltration | agent copie document interne dans outil externe | restrictions de sortie, DLP, logs |
| Action irréversible | suppression, publication, paiement | dry-run, approbation, rollback |
| Mémoire sensible | réutilisation de PII | rétention, minimisation, purge |
| Boucle autonome | actions répétées non contrôlées | limites, quotas, timeouts, supervision |
| Confusion d'autorité | document externe redéfinit règles | priorité instructions système/politique |

## Contrôles pédagogiques à enseigner

- Identifier ce que l'agent peut faire, pas seulement ce qu'il dit.
- Lister les permissions avant activation.
- Limiter lecture/écriture/envoi au strict nécessaire.
- Exiger validation humaine avant action à impact.
- Séparer brouillon et exécution.
- Vérifier logs et résultats.
- Prévoir arrêt, rollback et escalade.

## Scénarios pédagogiques

- Agent qui peut envoyer un email client à partir d'un résumé IA.
- Assistant connecté au drive qui récupère un document confidentiel par erreur.
- Ticket support contenant une prompt injection indirecte.
- Agent de code qui ouvre une MR avec secret dans diff.
- Workflow qui publie automatiquement une synthèse non vérifiée.

## Gates sécurité

Un scénario agentique doit être bloqué si la bonne réponse encourage :

- permissions larges sans justification ;
- action externe sans validation ;
- traitement d'un document non fiable comme instruction ;
- stockage mémoire de PII sans rétention ;
- absence d'audit d'action.

## Critères d'acceptation

- Les risques sont reliés à des choix utilisateurs.
- Les permissions et frontières de données sont explicites.
- Aucune automatisation dangereuse n'est encouragée.
- Les scénarios distinguent assistance, brouillon et action réelle.
