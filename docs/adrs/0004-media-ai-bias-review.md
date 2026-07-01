# ADR 0004 — Revue obligatoire des médias IA

## Statut

Acceptée.

## Contexte

Les captures de l'outil existant montrent un usage d'images générées ou synthétiques qui peut introduire des stéréotypes sans que les concepteurs s'en rendent compte. Les médias IA sont particulièrement risqués quand ils représentent des humains.

## Décision

Par défaut, aucun visage humain généré par IA n'est utilisé dans le corpus pédagogique, sauf si :

1. le scénario porte explicitement sur l'analyse d'un média synthétique ;
2. le média est signalé comme synthétique ;
3. la grille d'audit biais média est remplie ;
4. un relecteur biais approuve son usage.

## Conséquences

- Les médias décoratifs humains IA sont refusés.
- Les images abstraites ou schémas sont préférés.
- Les questions de deepfake doivent enseigner l'incertitude, pas le jugement d'apparence.
- Tout média a une provenance et un statut.

## Alternatives rejetées

- **Utiliser des images IA car rapides à produire** : risque de biais et de droits.
- **Laisser le modèle juger ses propres biais** : insuffisant.
- **Masquer que le média est synthétique** : contraire à la transparence pédagogique.
