# Sécurité & RGPD

## Classification des données

| Donnée                    | Sensibilité                 | Mesure                         |
| ------------------------- | --------------------------- | ------------------------------ |
| Corpus public validé      | faible à moyenne            | versionné, revue               |
| Corpus interne/politiques | interne                     | accès restreint                |
| Réponses utilisateur      | personnelle potentielle     | minimisation, rétention courte |
| Agrégats de cohorte       | faible si anonymisés        | seuil minimum de groupe        |
| Logs techniques           | peuvent contenir IP/session | minimiser, purger              |
| Médias pédagogiques       | droits/biais                | audit et provenance            |

## Principes RGPD

- Minimisation : ne collecter que ce qui sert la session.
- Finalité : entraînement pédagogique, pas évaluation RH cachée.
- Transparence : expliquer ce qui est stocké et visible.
- Droit à l'effacement : supprimer les données individuelles si stockées.
- Privacy by default : anonymat par défaut.
- Agrégats seulement pour l'organisation, avec seuil anti-réidentification.
- Positionnement relatif individuel uniquement privé, calculé depuis des distributions anonymisées.

## Données à ne pas demander

- Secrets, tokens, clés API.
- Données clients réelles.
- Données de santé, RH, financières personnelles.
- Documents internes réels dans les réponses libres.
- Photos/vidéos de collaborateurs.

## Authentification

MVP : mode anonyme ou pseudo-local.  
Organisation : OIDC devant, session serveur.

Biscuit peut être ajouté pour :

- délégations admin limitées ;
- accès à des corpus par organisation ;
- exports audités.

## Logs

À logger :

- identifiant de requête ;
- événement technique ;
- code erreur ;
- durée ;
- version corpus.

À ne pas logger :

- réponses libres ;
- identité complète ;
- tokens ;
- IP si non nécessaire ;
- prompts contenant données sensibles.

## Stockage local (meilleur score)

Le principe « meilleur score sans régression » (`docs/scoring.md`) s'appuie sur le
`localStorage` du navigateur, avec un contrat strict :

- Contenu autorisé : scores pédagogiques par question (clés namespacées, ex. `best_q_*`),
  rien d'autre.
- Aucune donnée nominative, aucun identifiant de personne, aucun token (la règle
  « pas de token en storage » ci-dessous reste entière).
- Aucune synchronisation ni persistance serveur : le stockage est client-only et sa
  suppression par l'utilisateur ou le navigateur est une perte acceptable.
- Vérifiable : l'export/inspection du storage ne doit révéler aucune PII.

## Checklist conformité (imports benchmark)

Enrichissements issus de `docs/benchmark-jeu-competitif.md` (Famille C) :

- Base légale explicitée et documentée (intérêt légitime pour l'entraînement pédagogique).
- Rétention **datée** pour toute donnée stockée, même technique.
- Droits des personnes rappelés dans l'écran d'introduction, avec voie de recours (CNIL).
- Si un « challenge » ponctuel est un jour organisé : règlement structuré préalable
  (éligibilité, rétention, litiges) et nouvelle ADR — cf. non-objectifs importés.
- Posture anti-triche : paradoxe pédagogique documenté (un entraînement _sur_ l'IA
  n'interdit pas l'IA ; aucun enjeu compétitif ne le justifie ici).

## Sécurité frontend

- Pas de token en `localStorage` ou `sessionStorage`.
- CSP restrictive.
- Pas de police ou script CDN tiers par défaut.
- Désactiver le tracking analytics tiers.
- Vérifier absence de PII dans exports client.

## Sécurité contenu

- Le corpus est une surface d'attaque : vérifier liens, HTML, Markdown, médias.
- Rendre le Markdown avec une allowlist stricte si utilisé.
- Ne jamais exécuter de contenu actif depuis le corpus.
- Les URLs de sources doivent être validées.

## Exécution et revue par défaut

Le flux nominal est local : build local, lancement sur `127.0.0.1`, revue humaine forte.

- Pas de plateforme cloud obligatoire.
- Pas de staging distant implicite.
- Pas de stockage serveur persistant en MVP.
- Toute diffusion hors localhost demande une décision humaine documentée.
- Monitoring et hébergement restent hors scope tant que le parcours local n'est pas validé.

## DPIA

Une DPIA légère devient nécessaire si :

- résultats individuels persistés ;
- usage organisationnel obligatoire ;
- croisement avec données RH ;
- scoring nominatif ;
- positionnement relatif calculé sur cohortes trop petites ;
- analyse de réponses libres.

Recommandation : éviter ces cas pour le MVP, sauf positionnement privé basé sur distributions anonymisées avec seuil minimum documenté.
