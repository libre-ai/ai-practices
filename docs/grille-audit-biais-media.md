# Grille d'audit des biais média IA

## Position par défaut

Pas de visage humain généré par IA dans le corpus pédagogique, sauf si le scénario porte explicitement sur l'analyse de média synthétique et si la revue de biais est approuvée.

## Pourquoi

Les images et vidéos générées peuvent introduire des biais sans intention explicite : stéréotypes ethniques, genre, âge, classe sociale, handicap, nationalité, profession, beauté, criminalité, compétence ou crédibilité.

## Fiche média

```yaml
id: media-001
status: draft|review|approved|blocked|retired
type: image|video|audio
origin: generated_ai|stock|internal|screenshot|synthetic_non_human
model: unknown
prompt: null
purpose: "Pourquoi ce média est nécessaire"
contains_human_like_person: true
consent_or_rights: "N/A ou preuve"
labels:
  synthetic_disclosed: true
  watermark_required: true
bias_review:
  stereotypes_detected: []
  representation_risks: []
  protected_attributes_inferred: []
  realism_risk: low|medium|high
  deepfake_confusion_risk: low|medium|high
  reviewer: TBD
  decision: approved|rework|blocked
  notes: "..."
```

## Checklist

### Nécessité

- Le média est-il indispensable à l'objectif pédagogique ?
- Un schéma neutre ou une capture anonymisée suffirait-il ?
- Le média décore-t-il seulement l'interface ? Si oui, refuser les visages IA.

### Transparence

- Le caractère synthétique est-il indiqué ?
- L'utilisateur peut-il comprendre ce qu'il doit analyser ?
- La question évite-t-elle de demander de juger une personne réelle ?

### Représentation

Vérifier si le média associe implicitement :

- une origine perçue à une fraude, incompétence ou dangerosité ;
- un genre à un rôle professionnel stéréotypé ;
- l'âge à la maîtrise ou non-maîtrise de l'IA ;
- l'apparence physique à la fiabilité ;
- un accent, vêtement ou décor à une caricature sociale ;
- une célébrité ou personne ressemblante sans droit.

### Qualité pédagogique

- La bonne réponse dépend-elle d'indices fiables ou de préjugés visuels ?
- La correction explique-t-elle les limites de la détection ?
- Le média évite-t-il "image propre = vraie" et "image bizarre = fausse" ?

## Décisions possibles

| Décision | Effet |
| --- | --- |
| `approved` | utilisable dans le contexte documenté |
| `rework` | média à régénérer/modifier, non publiable |
| `blocked` | média interdit dans le corpus |
| `retired` | retiré, conservé pour trace |

## Alternatives préférées

- Pictogrammes abstraits.
- Schémas faits main.
- Captures d'interface anonymisées.
- Documents fictifs sans personne.
- Médias synthétiques non humains.
- Texte descriptif sans image quand l'image n'apporte rien.
