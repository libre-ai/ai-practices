[English](README.md) · **Français**

> [!NOTE]
> **Réservé · futur foyer d'AI Practices** — reconstruit dans le dépôt de base canonique [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai) ([topologie multi-dépôts, ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).
> Ce dépôt rouvrira comme dépôt produit réel lorsque le propriétaire l'activera, consommant la base comme dépendance versionnée. Les fondations décrites ci-dessous sont **en cours de construction** — avec des liens vers le code qui existe déjà.

# AI Practices

**Formation professionnelle pour une pratique responsable et sourcée de l'IA.** Aider les apprenants à exercer un jugement explicite et borné dans des scénarios réalistes d'IA. Les apprenants possèdent leur progression hors ligne ; les relecteurs approuvent les versions d'activités ; les éditeurs publient du contenu curé — jamais classé automatiquement, jamais sans approbation humaine, jamais comme une liste de conformité.

Le cas canonique : _une équipe professionnelle exécute un scénario impliquant choix de modèle, sensibilité des données et contraintes juridiques, reçoit un retour explicite sourcé (non punitif), et vérifie son raisonnement face aux sources documentées et limitations connues._

## Ce qui la distingue

- **Possédée hors ligne, révisée une fois.** La progression de l'apprenant est locale et portable ; les versions d'activités sont immuables après approbation. Aucune surveillance continue, aucun profil apprenant côté serveur.
- **Retour sourcé, pas opinion de modèle.** Chaque indice référence une règle source approuvée (politique, documentation, étude de cas) — jamais utilise d'inférence de modèle non vérifiée comme clé de réponse.
- **Scénarios bornés, pas chat ouvert.** Les activités sont contraintes par schéma ; les réponses des apprenants doivent satisfaire l'interface définie. L'ambiguïté est documentée, jamais cachée.
- **Non punitif, focalisé audit.** L'issue est _en cours_, _complétée_ ou _arrêtée_ — pas un score succès/échec. Le retour préserve la réponse de l'apprenant et offre rejeu, jamais fabrique le succès.
- **Déterministe, dégradé gracieux.** Si un fournisseur de retour (modèle, service externe) est indisponible, des indices déterministes restent disponibles ; le retour non sourcé est marqué explicitement, jamais promu silencieusement au succès.

## État — spécifié publiquement, squelette produit complet

AI Practices est construit à partir de contrats verrouillés et d'un modèle de domaine. Il **n'est pas encore publié** ; la pratique locale hors ligne fonctionne et est prouvée, et la surface de révision/publication vient ensuite :

| Fondation                                                              | État         | Preuve                                                                                                                                                                           |
| ---------------------------------------------------------------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Définition d'activité et Résultat** — modèle de domaine              | ✅ construit | Schéma + domaine TypeScript, fixtures de contrats ([#163](https://github.com/libre-ai/libre-ai/pull/163))                                                                        |
| **Persistance locale des résultats** — en mémoire et IndexedDB         | ✅ construit | Adaptateurs avec tests unitaires, architecture hors ligne d'abord ([#183](https://github.com/libre-ai/libre-ai/pull/183), [#184](https://github.com/libre-ai/libre-ai/pull/184)) |
| **Propriété des données et suppression durable** — contrôles apprenant | ✅ construit | Export/réinitialisation, aucun résidu dans l'index côté serveur ([#190](https://github.com/libre-ai/libre-ai/pull/190))                                                          |
| **Interface utilisateur centré-client** — activité et progression      | ✅ construit | Squelette opérationnel, accessible au clavier, PWA React 19 ([#188](https://github.com/libre-ai/libre-ai/pull/188))                                                              |
| **API de révision et publication** — versionnage et approbation        | ⏳ suite     | Portail Biscuit, versions immuables, RLS pour isolation des locataires                                                                                                           |
| **Moteur de retour déterministe** — notation basée règles              | ⏳ suite     | Validateur TypeScript, réponses conformes au schéma, preuve d'accessibilité                                                                                                      |

Ce dépôt est une réserve publique, volontairement sans code produit jusqu'à l'activation (vague 4). **Cible de référence :** plateformes d'apprentissage professionnel (p. ex. DataCamp, Coursera) — différenciée par possession hors ligne d'abord, contenu révisé par humain, et sourçage explicite plutôt que recommandation algorithmique.

## Comment ça fonctionne

1. **Installer hors ligne** — l'apprenant télécharge une version d'activité approuvée (schéma + instructions + scénarios), stockée localement.
2. **Pratiquer avec retour** — l'apprenant soumet une réponse bornée ; le moteur valide le schéma et applique les règles de retour déterministes liées aux sources documentées. Les règles échouées montrent quelle règle a cassé et pourquoi. Le rejeu est toujours disponible.
3. **Exporter ou réinitialiser** — l'apprenant exporte un bundle de progression portable (preuve prête pour révision) ou supprime toutes les données locales sans implication serveur.
4. **Publier du contenu curé** — le relecteur valide source, licence, accessibilité et preuve de sécurité ; l'éditeur promeut la version immuable révisée à l'index d'activités.

## Architecture — assemblée à partir de briques interopérables

AI Practices est un produit assemblé à partir de briques versionnées indépendamment ; chacune est utilisable et testable seule, et le produit est leur composition (la cible multi-dépôts de [l'ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).

| Brique                                                | Rôle                                                      | Interface exposée / consommée                                                                                             |
| ----------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| **Définition d'activité v1 et Résultat v1** (schémas) | Contrats bornés pour activités                            | `activity-definition.v1.schema.json`, `activity-outcome.v1.schema.json`, fixtures de contrats                             |
| **`@libre-ai/ui`** (React 19)                         | Composants UI accessibles au clavier                      | App d'activité, flux de propriété des données, composable avec web-platform                                               |
| **`@libre-ai/web-platform`**                          | Fondation SSR / BFF Bun                                   | Gestionnaire de requêtes, document rendu côté serveur, environnement pour le client hors ligne d'abord                    |
| **Persistance locale des résultats** (TypeScript)     | Adaptateurs de stockage hors ligne d'abord                | Adaptateur en mémoire pour tests ; adaptateur IndexedDB pour persistance navigateur ; basé-port (testable indépendamment) |
| **Export de progression v1** (schéma)                 | Format de preuve portable                                 | `practice-progress-export.v1.schema.json` — données d'apprenant sans identité serveur, révisable                          |
| **OpenAPI : practices.v1.yaml** (réservé)             | Surface de révision et publication (non encore en direct) | Versionnage d'activités, portails d'approbation, isolation d'éditeur                                                      |

Le client hors ligne d'abord n'a aucune identité serveur ; il échange uniquement des versions d'activités immuables et des règles de retour sourcées. Tout consommateur qui parle les contrats d'activités peut rendu et pratiquer ; l'environnement hôte (serveur) est responsable des portails de publication.

## Où se déroule le travail

Tout le développement actif est dans le dépôt de base, sous :

- `apps/practices` — l'hôte produit (PWA hors ligne, coordination serveur)
- `packages/contracts/schemas` — les schémas verrouillés de définition d'activité, résultat et export de progression
- `contracts/openapi/practices.v1.yaml` — la surface API de révision/publication
- [`docs/apps/practices.md`](https://github.com/libre-ai/libre-ai/blob/main/docs/apps/practices.md) — le cahier des charges produit complet

Pour suivre l'avancement ou contribuer, ouvrez issues et pull requests dans [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai). Ce dépôt reste réservé jusqu'à son activation.

## Licence

EUPL-1.2.
