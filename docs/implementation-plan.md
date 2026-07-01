# Plan d'implémentation

## Principe

Découper pour permettre à des modèles légers d'implémenter sans redécider l'architecture.

Chaque paquet doit avoir :

- contexte ;
- fichiers autorisés ;
- interfaces attendues ;
- tests ;
- critères d'acceptation ;
- non-objectifs.

## Ordre recommandé

1. [`implementation/00-agent-operating-model.md`](implementation/00-agent-operating-model.md)
2. [`implementation/01-rust-workspace.md`](implementation/01-rust-workspace.md)
3. [`implementation/02-domain-contracts.md`](implementation/02-domain-contracts.md)
4. [`implementation/03-content-validation.md`](implementation/03-content-validation.md)
5. [`implementation/04-audit-engine.md`](implementation/04-audit-engine.md)
6. [`implementation/05-cli.md`](implementation/05-cli.md)
7. [`implementation/06-session-engine.md`](implementation/06-session-engine.md)
8. [`implementation/07-api-axum.md`](implementation/07-api-axum.md)
9. [`implementation/08-dioxus-pwa.md`](implementation/08-dioxus-pwa.md)
10. [`implementation/09-desktop-mobile.md`](implementation/09-desktop-mobile.md)

## Definition of done globale

- Tests verts.
- Pas de nouvelle dépendance non justifiée.
- Documentation mise à jour.
- Aucun contournement de sécurité.
- Aucun contenu `approved` invalide.
- Rapport de vérification reproductible.

## Ce qu'il ne faut pas déléguer à un modèle léger

- Décisions d'architecture.
- Choix de dépendance majeure.
- Règles RGPD.
- Validation finale de contenu.
- Acceptation de médias IA.
- Politique de scoring RH/organisation.
