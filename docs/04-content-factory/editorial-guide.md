---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../01-foundation/principles.md
  - ../01-foundation/glossary.md
  - ../03-domain-model/misconceptions.md
  - ./feedback-writing-guide.md
---

# Guide éditorial

## Objectif

Définir le ton, la structure et les règles d'écriture des contenus pédagogiques :
questions, scénarios, feedbacks, fiches réflexes et synthèses.

## Voix du produit

La voix de `rumble-ai-practices` est :

- claire ;
- précise ;
- prudente ;
- non culpabilisante ;
- orientée action ;
- transparente sur les limites.

Elle n'est pas :

- professorale ;
- infantilisante ;
- sensationnaliste ;
- marketing ;
- juridiquement péremptoire ;
- obsédée par le prompt.

## Règles de style

### Dire le contexte

Préférer :

> Dans ce contexte, le document peut encore contenir des commentaires internes ou
des métadonnées.

Éviter :

> Il ne faut jamais envoyer de documents à une IA.

### Dire le risque

Préférer :

> Le risque est d'exposer une donnée client ou un secret métier dans un outil non
autorisé.

Éviter :

> Ce choix est dangereux.

### Dire l'action

Préférer :

> Vérifiez la classification, retirez les éléments sensibles et utilisez l'outil
autorisé par votre organisation.

Éviter :

> Soyez prudent.

### Dire l'incertitude

Préférer :

> Si la politique interne ne précise pas ce cas, demandez une validation sécurité
ou DPO avant de poursuivre.

Éviter :

> C'est conforme.

## Structure recommandée d'un feedback

```text
1. Qualification du choix : correct / partiel / risqué / incorrect.
2. Risque ou bénéfice principal.
3. Raison contextualisée.
4. Action recommandée.
5. Source ou limite si nécessaire.
```

Exemple :

> Bonne intuition : demander une source aide. Mais ce n'est pas suffisant : une
source peut être inventée ou mal attribuée. Vérifiez le passage dans la source
primaire avant de réutiliser l'affirmation.

## Lexique recommandé

| Préférer | Éviter | Raison |
| --- | --- | --- |
| « dans ce contexte » | « toujours » | évite les absolus faux |
| « à vérifier » | « garanti » | préserve incertitude |
| « donnée personnelle » | « PII » seul | compréhensible et RGPD |
| « source primaire » | « lien » | insiste sur preuve |
| « outil autorisé » | « outil interne donc sûr » | évite raccourci |
| « revue humaine » | « validation automatique » | rappelle responsabilité |
| « axe de progression » | « niveau employé » | évite RH |

## Formulations interdites ou à éviter

- « Conforme RGPD » sans analyse.
- « 100 % sûr ».
- « L'IA ne se trompe pas si... ».
- « Il suffit de demander... ».
- « Toujours / jamais » sauf règle explicitement absolue.
- « Mauvais utilisateur » ou formulations culpabilisantes.
- « Score de maturité individuel ».
- « Meilleur prompt garanti ».

## Ton des corrections

### Correct

> Correct. Vous réduisez le risque principal en vérifiant le contenu réel avant
usage IA. C'est important car un brouillon peut contenir des commentaires,
versions ou métadonnées sensibles.

### Partiel

> Partiel. Utiliser un outil autorisé est important, mais il faut aussi vérifier
la classification du document et retirer les données non nécessaires.

### Risqué

> Risqué. Le fait qu'un document soit destiné à être publié ne garantit pas que
le brouillon actuel soit public ou dépourvu d'informations internes.

### Incorrect dangereux

> À éviter. Ce choix expose potentiellement des données personnelles ou secrets à
un outil non autorisé. Il faut bloquer l'action et demander une validation.

## Écrire sans dogmatisme

Une règle doit indiquer sa portée :

- outil externe vs outil autorisé ;
- donnée publique vs brouillon interne ;
- usage exploratoire vs publication ;
- faible enjeu vs décision à impact ;
- politique interne connue vs inconnue.

## Écrire pour le transfert

Chaque contenu doit aider l'utilisateur à agir hors du produit. Utiliser :

- actions concrètes ;
- formulation mémorisable mais non simpliste ;
- lien vers source ou politique ;
- exemple de prochaine étape.

## Biais et représentations

- Ne pas associer systématiquement un risque ou une erreur à un groupe.
- Varier métiers, rôles, niveaux hiérarchiques et contextes.
- Éviter les prénoms ou détails identifiants inutiles.
- Ne pas utiliser d'image humaine IA décorative.
- Si un scénario traite d'un biais, rendre l'objectif pédagogique explicite.

## Sources dans le texte utilisateur

L'interface ne doit pas noyer l'utilisateur dans la bibliographie. Préférer :

- une source principale ;
- une mention de politique interne ;
- un lien « pourquoi » ou « en savoir plus » ;
- un signal clair quand la source manque.

## Avant / après

### Avant

> Faux, il faut toujours vérifier les sources.

### Après

> Risqué. Demander une source ne suffit pas : elle peut être inventée ou ne pas
soutenir l'affirmation. Vérifiez le passage dans la source primaire avant de
réutiliser le contenu.

### Avant

> Bonne réponse : anonymisez les données.

### Après

> Bonne direction, mais attention : retirer les noms ne suffit pas toujours à
anonymiser. Réduisez les données au nécessaire et vérifiez si la politique
interne exige un outil autorisé ou une validation DPO.

## Critères d'acceptation

- Les formulations évitent humiliation et certitude excessive.
- Chaque feedback indique risque et action.
- Les sources sont intégrées naturellement.
- Les termes sont cohérents avec le glossaire.
- Les textes ne créent pas d'usage RH implicite.
