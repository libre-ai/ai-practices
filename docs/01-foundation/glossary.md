---
status: stable
owner: product-content
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ./source-policy.md
  - ../data-model.md
  - ../question-model.md
  - ../security-rgpd.md
---

# Glossaire

## Objectif

Réduire les ambiguïtés de vocabulaire entre produit, pédagogie, sécurité, RGPD et
technique.

Ce glossaire fournit des définitions de travail pour `rumble-ai-practices`. Il
ne remplace pas les textes juridiques, normes, politiques internes ou sources
scientifiques. Les termes réglementaires doivent être vérifiés contre les sources
primaires applicables.

## Règles d'usage

- Utiliser ces termes de manière cohérente dans questions, feedbacks, docs et
  schémas.
- Signaler les synonymes dangereux quand ils créent une confusion.
- Ne pas présenter une définition courte comme une vérité juridique complète.
- Ajouter une source dans la documentation thématique si le terme soutient une
  règle ou correction.

## IA et systèmes génératifs

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| IA | Ensemble de méthodes et systèmes qui automatisent ou assistent des tâches généralement associées au raisonnement, à la perception, à la classification ou à la génération. | Terme large ; préciser le type de système. |
| IA générative | Systèmes capables de produire texte, image, audio, vidéo, code ou structures à partir d'instructions, exemples ou contexte. | Ne pas assimiler à intelligence générale. |
| Modèle | Système entraîné ou configuré pour produire une sortie à partir d'une entrée. | Le modèle n'est pas l'application complète. |
| Modèle de fondation | Modèle généraliste entraîné à grande échelle, adaptable à de nombreux usages. | Les capacités varient fortement selon entraînement, outils et contexte. |
| LLM | Modèle de langage entraîné à prédire ou générer des séquences textuelles. | Peut produire un texte plausible mais faux. |
| Transformer | Architecture de modèle fondée notamment sur l'attention, largement utilisée dans les LLM modernes. | Utile pour comprendre certaines limites, pas nécessaire à chaque utilisateur. |
| Token | Unité de texte traitée par un modèle, souvent fragment de mot ou caractère. | Les limites de contexte se comptent en tokens, pas en pages humaines exactes. |
| Fenêtre de contexte | Quantité d'information qu'un modèle peut prendre en compte dans une interaction. | Long contexte ne signifie pas compréhension fiable de tout le contenu. |
| Sampling | Méthode de sélection de la prochaine sortie parmi plusieurs possibilités. | Peut augmenter variabilité et créativité, mais aussi erreurs. |
| Température | Paramètre influençant la variabilité des sorties. | Baisser la température ne garantit pas la vérité. |
| Fine-tuning | Adaptation d'un modèle par entraînement supplémentaire sur des données spécifiques. | N'est pas toujours nécessaire ; peut introduire risques de données. |
| Distillation | Technique visant à transférer le comportement d'un modèle vers un modèle plus petit. | Peut réduire coûts mais aussi capacités ou robustesse. |
| Quantization | Réduction de précision numérique pour rendre un modèle plus léger. | Peut affecter qualité selon usage. |
| MoE | Mixture of Experts : architecture activant seulement certaines parties du modèle selon l'entrée. | Les détails internes sont rarement visibles pour l'utilisateur final. |

## Prompting et interaction

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Prompt | Instruction ou contexte fourni à un système IA. | Un prompt n'est pas une garantie de vérité ni de sécurité. |
| Prompt engineering | Pratiques de formulation et structuration d'instructions pour améliorer les sorties. | Ne remplace pas vérification, politique interne ou contrôle humain. |
| System prompt | Instruction de niveau système qui cadre le comportement d'un assistant. | Peut être contournée ou entrer en conflit avec d'autres entrées. |
| Few-shot | Fournir quelques exemples pour guider la réponse. | Les exemples peuvent introduire biais ou fuite d'information. |
| Chain-of-thought | Technique ou comportement de raisonnement pas à pas. | Demander un raisonnement ne garantit pas sa validité. |
| Output format | Structure attendue de la sortie : JSON, tableau, résumé, etc. | Un format valide ne garantit pas un contenu exact. |
| Contexte utilisateur | Informations fournies par l'utilisateur pour cadrer la tâche. | Peut contenir PII, secrets ou données internes. |
| Instruction conflictuelle | Instructions incompatibles entre système, développeur, utilisateur ou document. | Peut produire des comportements inattendus. |

## RAG, sources et factualité

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| RAG | Architecture combinant récupération documentaire et génération. | La récupération ne garantit pas la fidélité de la réponse. |
| Retrieval | Recherche d'information pertinente dans un corpus. | Peut récupérer un document faux, obsolète ou hors contexte. |
| Embedding | Représentation numérique d'un texte, média ou item pour comparaison sémantique. | Similarité sémantique ne signifie pas exactitude. |
| Base vectorielle | Stockage optimisé pour rechercher des embeddings proches. | Ne remplace pas gouvernance documentaire. |
| Reranking | Réordonnancement de résultats récupérés. | Peut améliorer pertinence, pas garantir vérité. |
| Groundedness | Degré d'ancrage d'une réponse dans les sources fournies. | Ne vaut que si les sources sont pertinentes et correctement utilisées. |
| Faithfulness | Fidélité d'une réponse aux informations disponibles ou aux contraintes données. | Peut être distincte de la vérité globale. |
| Factualité | Conformité d'une affirmation aux faits vérifiables. | Les faits peuvent dépendre de date, juridiction ou contexte. |
| Source primaire | Document original ou autorité de référence. | À privilégier sur les résumés et citations de seconde main. |
| Source secondaire | Analyse, synthèse ou commentaire d'une source primaire. | Utile pour comprendre, insuffisante pour certaines obligations. |
| Source vendeur | Documentation ou communication d'un fournisseur. | Utile sur son produit, potentiellement orientée pour les généralités. |
| Source gap | Absence temporaire de source robuste pour une affirmation. | Autorisé en `draft`, bloquant pour publication sauf justification interne. |
| Citation inventée | Référence inexistante ou non vérifiable produite comme si elle existait. | Risque majeur pour confiance et conformité. |
| Mauvaise attribution | Source réelle mais qui ne soutient pas l'affirmation citée. | Plus difficile à détecter qu'une source inexistante. |

## Hallucinations, limites et erreurs

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Hallucination | Sortie non fidèle aux faits, sources ou contraintes, présentée de manière plausible. | Le terme recouvre plusieurs phénomènes distincts. |
| Plausibilité trompeuse | Impression de crédibilité produite par style, précision ou confiance apparente. | Un ton assuré n'est pas une preuve. |
| Omission critique | Absence d'une information importante pour décider. | Peut être aussi risquée qu'une erreur explicite. |
| Surconfiance | Présentation d'une réponse incertaine comme certaine. | À corriger par incertitude et vérification. |
| Obsolescence | Utilisation d'information dépassée. | Important pour droit, sécurité et capacités fournisseurs. |
| Généralisation abusive | Application d'une règle à un contexte où elle ne vaut pas. | Fréquent dans les conseils de prompt ou conformité. |

## Agents, outils et automatisation

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Agent IA | Système qui planifie ou enchaîne des actions/outils avec un modèle IA. | Risque accru aux frontières outil/donnée/autorisation. |
| Tool calling | Capacité d'un modèle ou orchestrateur à appeler un outil externe. | Les permissions et effets doivent être contrôlés. |
| MCP | Protocole de connexion à des outils/contextes utilisé dans certains environnements IA. | Un connecteur élargit la surface de risque. |
| Orchestration | Coordination d'étapes, modèles, outils et mémoires. | Les erreurs peuvent se propager entre étapes. |
| Mémoire agentique | Stockage d'informations entre interactions ou tâches. | Peut retenir des données sensibles si mal gouvernée. |
| Human-in-the-loop | Intervention humaine dans un processus automatisé. | Doit être réelle et située au bon moment. |
| Human-on-the-loop | Supervision humaine d'un système qui agit principalement seul. | Insuffisant pour certains risques élevés. |
| Permission outil | Droit accordé à un agent : lire, écrire, envoyer, supprimer, exécuter. | Principe du moindre privilège. |

## Sécurité

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Prompt injection | Instruction malveillante ou non souhaitée visant à détourner le comportement du système IA. | Peut venir de l'utilisateur ou d'un document récupéré. |
| Prompt injection indirecte | Injection contenue dans une source externe : page web, email, PDF, ticket, dépôt. | Critique pour RAG et agents. |
| Jailbreak | Tentative de contourner les règles ou garde-fous d'un modèle. | Ne pas enseigner de procédures offensives inutiles. |
| Data exfiltration | Extraction non autorisée de données. | Peut passer par sorties, outils, logs ou mémoire. |
| Secret | Information confidentielle technique, commerciale ou organisationnelle. | Ne doit pas être envoyée à un outil IA non autorisé. |
| Secret scanning | Détection automatique de secrets dans code, logs ou fichiers. | Dernier filet, pas excuse pour exposer. |
| Supply chain | Chaîne de dépendances, modèles, données, outils et services utilisés. | Les risques viennent aussi de composants indirects. |
| Least privilege | Principe du moindre privilège. | Donner à un agent seulement les droits nécessaires. |
| Audit trail | Trace vérifiable d'actions, décisions et accès. | Doit éviter PII ou secrets inutiles. |
| Red teaming | Recherche structurée de failles ou comportements indésirables. | Complément à la revue, pas preuve absolue. |

## Données, RGPD et conformité

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Donnée personnelle | Information se rapportant à une personne identifiée ou identifiable. | Le contexte peut rendre une donnée identifiable. |
| PII | Personally Identifiable Information ; terme souvent utilisé pour données identifiantes. | Ne couvre pas toujours exactement le champ RGPD. |
| Donnée sensible | Catégories particulières ou données à risque élevé selon contexte. | Santé, opinions, biométrie, etc. nécessitent prudence renforcée. |
| Minimisation | Principe consistant à limiter les données au nécessaire. | Fondamental pour prompts, logs et analytics. |
| Base légale | Fondement juridique d'un traitement de données personnelles. | À vérifier avec DPO/juridique. |
| Finalité | Objectif déterminé d'un traitement. | Interdit de réutiliser sans base compatible. |
| Rétention | Durée de conservation des données. | Doit être courte, justifiée et appliquée. |
| Anonymisation | Transformation empêchant raisonnablement toute réidentification. | Difficile ; irréversible attendue. |
| Pseudonymisation | Remplacement d'identifiants directs par des pseudonymes. | Reste une donnée personnelle en RGPD. |
| DPIA / AIPD | Analyse d'impact relative à la protection des données. | Requise pour certains traitements à risque élevé. |
| DPO | Délégué à la protection des données. | Interlocuteur pour questions RGPD. |
| AI Act | Règlement européen sur l'intelligence artificielle. | Obligations dépendantes du rôle et du niveau de risque. |
| DORA | Cadre européen de résilience opérationnelle numérique pour le secteur financier. | Applicable selon secteur et organisation. |
| Politique interne | Règle propre à une organisation. | Peut être plus restrictive qu'une règle générale. |

## Biais, équité et médias

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Biais | Écart systématique pouvant produire représentation ou décision injuste. | Peut venir données, modèle, prompt, contexte ou revue. |
| Équité | Recherche d'un traitement juste selon critères pertinents. | Les définitions d'équité peuvent entrer en tension. |
| Stéréotype | Association simplificatrice attribuée à un groupe. | Peut être narratif ou visuel. |
| Proxy discriminant | Variable apparemment neutre corrélée à une caractéristique protégée. | Peut créer discrimination indirecte. |
| Média synthétique | Image, audio, vidéo ou contenu visuel généré ou modifié par IA. | Exige provenance et audit selon usage. |
| Deepfake | Média synthétique représentant une personne de manière trompeuse. | Risque fort de manipulation, consentement et confiance. |
| Provenance média | Informations sur origine, génération, modification et droits d'un média. | Indispensable pour audit. |
| Revue biais | Analyse humaine ou assistée des représentations et impacts. | Ne doit pas rationaliser un biais inutile. |

## Pédagogie et évaluation

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Compétence observable | Capacité démontrable par un comportement dans un scénario. | Préférer à une connaissance déclarative seule. |
| Objectif pédagogique | Résultat d'apprentissage attendu. | Doit être mesurable sans surveillance abusive. |
| Feedback pédagogique | Explication qui aide à comprendre et agir. | Doit inclure risque, raison et action. |
| Retrieval practice | Pratique consistant à récupérer activement une connaissance. | À utiliser avec contexte, pas seulement mémorisation. |
| Spacing | Espacement des rappels dans le temps. | Demande prudence si tracking utilisateur. |
| Métacognition | Capacité à évaluer ce qu'on sait, ignore ou doit vérifier. | Centrale face aux sorties IA plausibles. |
| Charge cognitive | Effort mental imposé par une tâche. | Trop de détails nuit à l'apprentissage. |
| Transfert | Capacité à appliquer un apprentissage dans un autre contexte. | Objectif principal des scénarios professionnels. |
| Taxonomie de Bloom | Classification de niveaux cognitifs. | Utile mais pas suffisante pour juger compétence pro. |
| Scoring pédagogique | Score conçu pour guider l'apprentissage. | Ne doit pas devenir un score RH. |
| Réponse partielle | Choix qui contient une bonne intuition mais manque une condition importante. | À distinguer de réponse dangereuse. |
| Distracteur | Choix incorrect mais plausible dans une question. | Ne doit pas être un piège injuste. |

## Produit, contenu et gouvernance

| Terme | Définition de travail | Point de vigilance |
| --- | --- | --- |
| Corpus | Ensemble versionné de questions, scénarios, médias, sources et futurs concepts. | Doit être validable localement. |
| Content-as-data | Principe selon lequel le contenu est stocké comme donnée, pas codé dans l'UI. | Permet schémas, audits et versionnement. |
| Content Factory | Chaîne de production et revue des contenus. | Produit des brouillons, pas des approbations automatiques. |
| Quality gate | Critère automatique ou humain qui autorise, avertit ou bloque. | Les blockers doivent être fail-closed. |
| Revue humaine | Validation par une personne responsable avant publication. | Non substituable par une note automatique. |
| `draft` | Brouillon non publiable. | Statut par défaut des contenus nouveaux. |
| `review` | Prêt à être relu humainement. | Non publiable tant que non approuvé. |
| `approved` | Validé pour publication selon gouvernance. | Demande preuves de revue. |
| `blocked` | Non publiable tant qu'un risque n'est pas résolu. | Doit indiquer raison/action. |
| `retired` | Retiré mais conservé pour traçabilité. | Ne doit plus être servi par défaut. |
| Cohorte anonyme | Groupe permettant une comparaison agrégée sans identification individuelle. | Taille minimale et rétention nécessaires. |
| k-anonymat | Propriété imposant au moins k individus dans un groupe avant affichage. | Réduit mais ne résout pas tous les risques de réidentification. |
| Leaderboard | Classement d'utilisateurs. | Hors périmètre nominatif du produit. |
| Badge | Marqueur de progression ou accomplissement. | Doit éviter pression ou usage RH. |

## Synonymes et confusions dangereuses

| Confusion | Pourquoi c'est dangereux | Formulation recommandée |
| --- | --- | --- |
| « anonymisé » pour « pseudonymisé » | Sous-estime les obligations RGPD. | Dire pseudonymisé si réidentification possible. |
| « source citée » pour « source vérifiée » | Une citation peut être fausse ou non pertinente. | Dire source vérifiée ou source à vérifier. |
| « IA interne » pour « usage sûr » | Interne ne garantit pas droits, logs, rétention ou qualité. | Préciser outil, données, politique et contrôles. |
| « prompt sécurisé » pour « système sécurisé » | Le prompt ne contrôle pas tous les risques. | Parler de mesures techniques, processus et revue. |
| « score de maturité » pour « performance individuelle » | Peut créer usage RH implicite. | Parler d'axes de progression privés ou agrégés. |
| « RAG = pas d'hallucination » | Le RAG peut mal récupérer ou mal citer. | Dire RAG peut améliorer l'ancrage mais doit être évalué. |
| « humain dans la boucle » pour « responsabilité réglée » | La revue peut être symbolique ou trop tardive. | Préciser moment, compétence et pouvoir d'arrêt. |

## Termes à sourcer en priorité

Les termes suivants doivent être rattachés aux sources primaires ou scientifiques
dans les documents de recherche avant usage comme base de correction :

- obligations RGPD, AI Act, DORA ;
- anonymisation et pseudonymisation ;
- hallucination, factualité, groundedness ;
- prompt injection indirecte ;
- méthodes d'évaluation LLM ;
- retrieval practice, spacing, charge cognitive ;
- biais et équité algorithmique.

## Critères d'acceptation

- Chaque terme a une définition courte.
- Les synonymes dangereux sont signalés.
- Les termes instables indiquent leur niveau d'incertitude ou demandent une
  source thématique.
- Les définitions ne remplacent pas les sources normatives.
- Le glossaire couvre produit, pédagogie, IA, sécurité, RGPD, biais et runtime.

## Statut de revue

Ce document a passé la revue humaine `foundation-review` et sert de référence
canonique pour la Vague A.
