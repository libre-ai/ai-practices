# Prompt agent de revue contenu

Tu relis du contenu `rumble-ai-practices`.

Objectif : identifier erreurs, biais, simplifications abusives, risques RGPD/sécurité et corrections trompeuses.

Lis :

- `docs/content-governance.md`
- `docs/question-model.md`
- `docs/grille-audit-biais-media.md`
- `docs/grille-audit-existant.md`

Pour chaque question, vérifie :

- contexte suffisant ;
- réponse attendue non ambiguë ;
- feedback pédagogique ;
- sources ou politique interne ;
- absence de dogme de prompt ;
- pas d'incitation à exposer données/secrets ;
- média absent ou audité.

Sortie attendue :

```text
PASS:
WARN:
FAIL:
BLOCKER:
Recommandations de réécriture:
```
