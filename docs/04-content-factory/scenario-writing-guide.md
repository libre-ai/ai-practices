---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../03-domain-model/risk-model.md
  - ../03-domain-model/competency-model.md
  - ../03-domain-model/misconceptions.md
  - ../grille-audit-biais-media.md
  - ./editorial-guide.md
---

# Guide d'écriture des scénarios

## Objectif

Définir comment construire des situations professionnelles réalistes,
pédagogiques et non biaisées.

Un scénario est bon si l'utilisateur peut se reconnaître dans la décision sans
qu'aucune donnée réelle, personne identifiable ou stéréotype ne soit nécessaire.

## Canevas de scénario

```yaml
role: collaborateur marketing
task: faire relire un brouillon par un assistant IA
data_context: document destiné à publication mais encore en brouillon
tool_context: assistant IA externe non explicitement autorisé
constraint: délai court avant publication
stakes: risque de commentaires internes, métadonnées ou roadmap
ambiguity_level: low
```

## Éléments obligatoires

| Élément | Question |
| --- | --- |
| Rôle | Qui agit ? |
| Tâche | Que veut faire la personne ? |
| Données | Quel type d'information est manipulé ? |
| Outil IA | Interne, externe, autorisé, inconnu ? |
| Contrainte | Pourquoi la décision n'est-elle pas triviale ? |
| Enjeu | Quel impact si mauvais choix ? |
| Signal pédagogique | Quel risque/compétence teste-t-on ? |

## Niveaux d'ambiguïté

| Niveau | Usage | Exemple |
| --- | --- | --- |
| `low` | débutant | document clairement interne |
| `medium` | intermédiaire | document destiné publication mais brouillon |
| `high` | avancé | outil interne avec droits larges et source contradictoire |

L'ambiguïté doit entraîner le jugement, pas masquer la bonne réponse.

## Types de scénarios

| Type | Exemple | Axes fréquents |
| --- | --- | --- |
| Données | envoyer un contrat à un assistant | confidentialité, RGPD |
| Source | réutiliser une citation IA | source verification, hallucination |
| Code/sécurité | copier un log dans un assistant | security, data confidentiality |
| RAG | réponse fondée sur document interne | source verification, business_context |
| Agent | outil peut envoyer email | security, human_responsibility |
| Média | image générée pour communication | media_synthetic, bias_fairness |
| Gouvernance | demande de score manager | privacy_rgpd, no-RH |

## Réalisme sans données réelles

Utiliser :

- métiers génériques ;
- documents fictifs ;
- dates non sensibles ;
- entreprises fictives ;
- tokens manifestement faux si exemple nécessaire ;
- données synthétiques minimales.

Éviter :

- noms réels ;
- clients réels ;
- incidents internes identifiables ;
- captures d'écran ;
- logs réalistes contenant IP/email/token ;
- détails qui permettent une réidentification.

## Biais à éviter

- Associer systématiquement un rôle junior à l'erreur.
- Mettre toujours sécurité/juridique dans le rôle du blocage.
- Utiliser des prénoms ou origines pour signaler compétence/incompétence.
- Présenter un métier comme naturellement irresponsable.
- Générer des images humaines décoratives.
- Créer un scénario où un groupe est le problème plutôt que le système/processus.

## Variantes de contexte

Pour augmenter la couverture, varier :

- métiers : marketing, support, produit, RH, juridique, dev, direction ;
- types de données : public, interne, confidentiel, personnel, secret ;
- outils : assistant texte, RAG, IDE IA, agent, générateur image ;
- finalités : brouillon, publication, décision, support, veille ;
- pression : délai, demande manager, client, incident.

## Exemples rapides

### Bon scénario

> Vous préparez une note interne avec une statistique fournie par un assistant IA.
> L'assistant cite un rapport public, mais vous n'avez pas encore ouvert le lien.
> Que faites-vous avant d'envoyer la note ?

Pourquoi c'est bon : rôle, tâche, source, action, risque.

### Mauvais scénario

> L'IA donne une source. Est-ce fiable ?

Pourquoi c'est faible : pas de contexte, pas d'enjeu, réponse trop générale.

## Checklist auteur

- Le rôle est-il utile ?
- La tâche est-elle réaliste ?
- Les données sont-elles fictives et non sensibles ?
- L'outil IA est-il suffisamment décrit ?
- Le risque principal est-il testable ?
- L'ambiguïté est-elle contrôlée ?
- Le scénario évite-t-il les stéréotypes ?
- Le contexte suffit-il à déterminer la meilleure réponse ?

## Critères d'acceptation

- Aucune donnée personnelle réelle.
- Les stéréotypes sont évités.
- L'ambiguïté sert l'apprentissage, pas le piège.
- Le scénario relie rôle, donnée, outil, risque et compétence.
- Le scénario peut être revu sans contexte externe secret.
