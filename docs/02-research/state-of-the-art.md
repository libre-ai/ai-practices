---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./bibliography.md
  - ../01-foundation/source-policy.md
  - ../03-domain-model/taxonomy.md
  - ../03-domain-model/competency-model.md
---

# État de l'art

## Objectif

Fournir une revue structurée des connaissances nécessaires pour concevoir un
produit sérieux d'entraînement aux pratiques IA. Ce document est un hub : il
résume les consensus utiles au produit et renvoie aux dossiers thématiques pour
le détail.

## Méthode de revue

La revue utilise la hiérarchie de `../01-foundation/source-policy.md` :

1. sources normatives et institutionnelles pour obligations, gouvernance et
   sécurité ;
2. littérature scientifique pour phénomènes IA et principes d'apprentissage ;
3. sources techniques mainteneur pour menaces et patterns ;
4. sources vendeurs seulement pour capacités ou politiques de leurs produits.

Chaque synthèse distingue :

- **consensus opérationnel** : suffisamment robuste pour guider un scénario ;
- **incertitude** : utile mais à formuler prudemment ;
- **controverse** : à éviter comme vérité pédagogique simple ;
- **implication corpus** : ce que cela change pour questions/feedbacks.

## Carte des domaines

| Domaine | Documents détaillés | Source IDs candidates |
| --- | --- | --- |
| Littératie IA | `ai-literacy.md` | `source-long-magerko-ai-literacy-2020` |
| Cognition/apprentissage | `cognitive-science.md` | `source-sweller-cognitive-load`, `source-roediger-karpicke-2006` |
| IA générative | `generative-ai.md` | `source-bommasani-foundation-models-2021`, `source-bender-stochastic-parrots-2021` |
| LLM | `llm.md` | `source-vaswani-attention-2017`, `source-brown-gpt3-2020` |
| RAG | `rag.md` | `source-lewis-rag-2020`, `source-ragas` |
| Agents | `agents.md` | `source-owasp-llm-top10`, `source-mitre-atlas` |
| Hallucinations | `hallucinations.md` | `source-ji-hallucination-survey-2023` |
| Évaluation | `evaluation.md` | `source-helm-2023`, `source-nist-ai-rmf-1-0` |
| Cybersécurité | `cybersecurity.md` | `source-owasp-llm-top10`, `source-mitre-atlas` |
| Gouvernance | `governance-regulation.md` | `source-eu-ai-act-2024-1689`, `source-eu-gdpr-2016-679` |

## Synthèse exécutive

### Consensus opérationnels

1. Une sortie IA générative doit être traitée comme une proposition, pas comme
   une preuve.
2. La qualité d'un prompt ne supprime pas les risques de données, sources,
   hallucination, biais ou responsabilité.
3. Le RAG peut améliorer l'ancrage documentaire, mais ne garantit ni vérité ni
   bonne attribution.
4. Les agents et outils augmentent le risque parce qu'ils peuvent lire, écrire,
   envoyer ou transformer des données.
5. Les risques IA sont socio-techniques : modèle, données, interface, workflow,
   organisation et utilisateur interagissent.
6. La formation efficace doit entraîner des décisions contextualisées, pas des
   définitions isolées.
7. Le scoring pédagogique doit servir la remédiation et non la comparaison
   individuelle.

### Incertitudes à formuler prudemment

- Les performances déclarées d'un modèle varient selon tâches, langues,
  contexte, outils, version et données.
- Les benchmarks publics ne prédisent pas toujours la fiabilité dans un workflow
  métier donné.
- Les métriques automatiques de factualité, groundedness ou sécurité sont utiles
  mais incomplètes.
- Les effets de formation dépendent du public, du contexte, du feedback et de la
  répétition.

### Controverses ou zones instables

- L'usage de LLM-as-judge comme substitut à la revue humaine.
- Les promesses d'agents autonomes sûrs avec permissions larges.
- Les comparaisons générales entre fournisseurs sans méthodologie indépendante.
- Les affirmations de conformité fondées seulement sur une documentation vendeur.

## Implications pour `rumble-ai-practices`

| Constat | Traduction produit |
| --- | --- |
| Les sorties IA peuvent être plausibles et fausses | scénarios de vérification, sources primaires, feedback d'incertitude |
| Les données exposées dans prompts/outils/logs créent un risque | questions données/PII/secrets, gates blockers |
| Le RAG ne résout pas tout | questions sur obsolescence, mauvaise attribution, documents non autorisés |
| Les agents élargissent la surface d'action | scénarios permissions, tool calling, validation avant action |
| Les formations superficielles créent illusion de compétence | scénarios contextualisés, progression, feedback explicatif |
| Les organisations veulent des signaux | agrégats anonymes par axe, pas de classement individuel |

## Axes de recherche à maintenir

1. Littératie IA professionnelle.
2. Pédagogie des réflexes et transfert.
3. Fiabilité, hallucinations, factualité et évaluation.
4. Sécurité applicative LLM, agents et RAG.
5. Gouvernance, RGPD, AI Act, normes et politiques internes.
6. Biais, équité, médias synthétiques et provenance.
7. Mesure produit respectueuse de la vie privée.

## Anti-patterns de formation observés

- Enseigner des prompts sans enseigner la vérification.
- Réduire RGPD à « ne pas mettre de noms ».
- Dire « demande une source » sans apprendre à vérifier cette source.
- Présenter RAG comme solution aux hallucinations.
- Encourager l'automatisation agentique sans modèle de permissions.
- Utiliser vitesse, leaderboard ou score global comme mesure de maturité.
- Publier un corpus sans date de revue.

## Open questions avant `stable`

- Quelles références AI literacy doivent devenir sources primaires du framework
  maison ?
- Quelle granularité de taxonomie est optimale : 100, 250 ou 500 concepts ?
- Quelles métriques automatiques sont suffisamment fiables pour des warnings,
  sans jamais approuver ?
- Comment représenter les sources internes sans exposer de secret ?
- Quels seuils de revue périodique par domaine doivent être codés dans le CLI ?

## Critères d'acceptation

- Chaque section distingue consensus, incertitudes et controverses.
- Les affirmations structurantes renvoient à des source IDs candidates.
- Les sources vendeurs sont clairement séparées dans la bibliographie.
- Les implications pour le corpus sont explicites.
- Le document sert de hub vers les dossiers thématiques.
