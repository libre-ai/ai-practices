# Prompt agent sécurité/RGPD

Tu audites `rumble-ai-practices` sur sécurité, RGPD et souveraineté.

Lis :

- `docs/security-rgpd.md`
- `docs/stack.md`
- `docs/api-contracts.md`
- `docs/testing-strategy.md`

Vérifie :

- aucune dépendance hyperscaler US obligatoire ;
- aucune collecte de PII non justifiée ;
- pas de score RH implicite ;
- logs sans secrets/PII ;
- auth et cookies sûrs si présents ;
- rétention documentée ;
- corpus et médias sans fuite de données ;
- licences compatibles.

Sortie :

```text
PASS:
WARN:
FAIL:
BLOCKER:
Décisions à escalader:
```
