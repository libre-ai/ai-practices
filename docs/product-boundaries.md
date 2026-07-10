# Frontières produit

## Appartenance écosystème

`rumble-ai-practices` appartient à la couche **Rumble — Product**.

Il possède l'expérience utilisateur, les scénarios pédagogiques, le feedback et la gouvernance du corpus pour l'entraînement aux pratiques IA.

## Possède

- Parcours d'entraînement aux pratiques IA.
- Corpus de questions/scénarios et fiches réflexes.
- Modèle de scoring pédagogique.
- Audit de biais des médias utilisés dans le produit.
- UX de session, progression et synthèse.
- Validation locale du corpus.
- Contrats API nécessaires au produit.

## Ne possède pas

- Un LMS générique.
- Un moteur de génération IA générique.
- Une plateforme de quiz généraliste.
- L'orchestration agentique : appartient à Bolt / `cos-matic`.
- L'extraction documentaire générique : candidat Wrench si le besoin grossit.
- Le stockage/provenance générique : candidat Gear.
- L'évaluation RH des collaborateurs.

### Non-objectifs importés (benchmark du jeu compétitif de référence)

Mécaniques compétitives explicitement hors produit, par décision de conception
(`docs/benchmark-jeu-competitif.md`, ADR 0003) :

- Vitesse notée / chrono par question.
- Vies-sanction / _game over_.
- Tickets / rationnement de l'entraînement.
- Ligues inter-joueurs, promotion/relégation.
- Multiplicateurs de score, note globale.
- Leaderboard et prix nominatifs.

Toute réintroduction passe par une nouvelle ADR (et une analyse d'impact si des
données personnelles sont en jeu).

## Tests de frontière

| Si la fonctionnalité...                      | Décision                                     |
| -------------------------------------------- | -------------------------------------------- |
| change l'expérience d'entraînement IA        | reste ici                                    |
| valide la qualité d'une question du corpus   | reste ici                                    |
| extrait du texte depuis PDF/DOCX/vidéo       | candidat Wrench                              |
| orchestre des agents de génération/revue     | candidat Bolt                                |
| stocke des artefacts signés/réutilisables    | candidat Gear                                |
| anime des sessions live génériques           | considérer `rumble-lm`                       |
| devient un contenu de formation réutilisable | exporter comme corpus, pas réinventer un LMS |

## Challenge structurel

Le produit chevauche partiellement `rumble-lm`. La justification d'un repo séparé tient seulement si :

- la gouvernance de contenu IA-practices est spécifique ;
- les audits de biais média sont centraux ;
- le modèle de scoring pédagogique diffère d'une session live générique ;
- l'outil doit pouvoir tourner seul et servir de référence qualité.

Sinon, il faut envisager `rumble-ai-practices` comme **pack de contenu + module spécialisé** pour `rumble-lm` plutôt qu'un produit complet.

Décision recommandée : démarrer en repo séparé contract-first, puis extraire les primitives réutilisables après preuve produit.
