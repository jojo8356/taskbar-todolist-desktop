---
stepsCompleted:
  - step-01-init
  - step-02-discovery
  - step-02b-vision
  - step-02c-executive-summary
  - step-03-success
  - step-04-journeys
  - step-05-domain
  - step-06-innovation
  - step-07-project-type
  - step-08-scoping
  - step-09-functional
  - step-10-nonfunctional
  - step-11-polish
  - step-12-complete
releaseMode: "phased"
inputDocuments:
  - /home/Johan/Documents/taskbar-todolist-desktop/_bmad-output/planning-artifacts/product-brief-taskbar-todolist.md
  - /home/Johan/Documents/taskbar-todolist-desktop/docs/product.md
  - /home/Johan/Documents/taskbar-todolist-mobile/README.md
  - /home/Johan/Documents/taskbar-todolist-mobile/docs/product.md
  - /home/Johan/Documents/taskbar-todolist-org/README.md
  - /home/Johan/Documents/taskbar-todolist-org/docs/architecture.md
  - /home/Johan/Documents/taskbar-todolist-org/docs/product-roadmap.md
  - /home/Johan/Documents/taskbar-todolist-org/docs/sync-model.md
documentCounts:
  productBriefs: 1
  research: 0
  brainstorming: 0
  projectDocs: 7
workflowType: "prd"
classification:
  projectType: "desktop_app"
  domain: "general"
  complexity: "low"
  projectContext: "greenfield"
  notes:
    - "Desktop Linux first with Tauri."
    - "Mobile app is a companion surface, not the primary project type."
    - "MVP is personal, local-first, and limited to simple tasks."
    - "Sync should be local network or USB first, with no mandatory cloud account."
visionDiscovery:
  vision: "Une todolist personnelle proche du systeme, accessible depuis la barre des taches, pour capturer et nettoyer ses taches sans casser le flux de travail."
  differentiator: "Le produit ne cherche pas a etre une grosse app de productivite; il optimise les actions les plus frequentes: ajouter, voir, supprimer."
  coreInsight: "Pour des taches simples, l'important n'est pas d'avoir plus de fonctions, mais de reduire le temps entre 'j'y pense' et 'c'est note'."
---

# Product Requirements Document - Taskbar Todolist

**Author:** Johan
**Date:** 2026-05-17

## Executive Summary

Taskbar Todolist est une application personnelle de gestion de taches simples, concue d'abord pour Linux avec Tauri. Le produit permet de capturer, consulter et supprimer des taches depuis la barre des taches/system tray, sans ouvrir une application complete ni interrompre le flux de travail. L'interface complete existe pour les modifications ponctuelles, tandis que l'app mobile sert de compagnon synchronise pour retrouver et gerer les memes taches hors du poste desktop.

En Phase 1, l'interaction principale est volontairement stricte: l'utilisateur clique sur l'icone de la barre des taches, un petit panneau s'ouvre, un input reste en haut du panneau pour ajouter rapidement une ou plusieurs taches avec `Entree`, la liste des taches est visible dessous, et une icone poubelle a droite de chaque tache permet de la supprimer.

Le probleme cible est la friction des todolists classiques pour les petites taches quotidiennes. Pour un usage personnel, les fonctions avancees comme projets, tags, priorites complexes, collaboration ou integrations creent souvent plus de bruit que de valeur. Le MVP se concentre donc sur une liste simple, local-first, utilisable sans compte et sans cloud obligatoire.

La synchronisation doit rester pragmatique: les donnees vivent localement sur les appareils, puis sont echangees via reseau local ou USB. Internet peut etre utile plus tard, mais ne doit pas etre une dependance centrale du produit.

### What Makes This Special

Le produit se differencie par son point d'entree: la barre des taches Linux est la surface principale, pas une fenetre d'application traditionnelle. Les actions les plus frequentes, ajouter et supprimer, doivent etre disponibles immediatement. La modification est volontairement separee dans une UI plus complete pour garder l'experience rapide legere.

L'insight central est que, pour des taches simples, la valeur ne vient pas d'un nombre eleve de fonctionnalites mais de la reduction du temps entre "j'y pense" et "c'est note". Taskbar Todolist assume un perimetre personnel, minimaliste et local-first au lieu de devenir une suite de productivite generaliste.

## Project Classification

- **Project Type:** Desktop app, avec app mobile compagnon.
- **Primary Platform:** Linux.
- **Desktop Technology:** Tauri.
- **Domain:** General productivity / personal task management.
- **Complexity:** Low.
- **Project Context:** Greenfield.
- **MVP Scope:** Taches simples, ajout, suppression, modification basique et stockage local sur desktop Linux. Mobile et synchronisation locale/USB sont en Phase 2.

## Product Definitions

### Tache simple

Une tache simple contient uniquement les donnees necessaires au MVP: identifiant stable, texte, statut `a faire` ou `terminee`, date de creation, date de modification, et marqueur de suppression optionnel pour la synchronisation. Les projets, tags, priorites complexes, sous-taches, commentaires, pieces jointes et rappels sont hors scope MVP.

### Synchronisation retenue

Les anciens documents d'organisation mentionnent des pistes backend/cloud. Pour ce PRD, la decision active est differente: la Phase 1 ne depend d'aucune synchronisation, et la Phase 2 vise une synchronisation locale ou USB sans compte cloud obligatoire. Toute architecture future doit traiter les references backend/cloud historiques comme non retenues pour le MVP et la Phase 2.

## Success Criteria

### User Success

- L'utilisateur peut ajouter une tache depuis la barre des taches Linux en moins de 5 secondes.
- L'utilisateur peut supprimer une tache depuis la barre des taches sans ouvrir l'UI complete.
- L'utilisateur peut consulter ses taches actives depuis le tray sans changer de contexte de travail.
- L'utilisateur peut modifier le texte ou le statut d'une tache depuis une UI dediee.
- L'utilisateur peut utiliser l'app desktop sans connexion internet.
- L'utilisateur peut retrouver sur mobile les taches creees sur desktop apres une synchronisation locale ou USB.

### Business Success

- Le MVP est utile pour un usage personnel quotidien avant toute ambition publique.
- Le produit remplace efficacement une note rapide ou une todolist simple pour les taches courtes.
- Le projet reste suffisamment simple pour etre maintenable par une seule personne.
- Aucune dependance a un compte cloud ou a une infrastructure SaaS n'est requise pour valider le MVP.

### Technical Success

- L'app desktop est construite avec Tauri et fonctionne sur Linux.
- Les taches sont persistees localement.
- Le modele de tache reste minimal: identifiant, texte, statut, dates de creation/modification, suppression eventuelle.
- La synchronisation locale ou USB transfere les taches sans doublons.
- Une erreur de sync ne doit pas faire perdre les donnees locales.
- L'app mobile peut lire, ajouter, modifier et supprimer les memes taches simples.

### Measurable Outcomes

- Temps cible d'ajout desktop: moins de 5 secondes.
- Temps cible de suppression desktop: moins de 2 actions utilisateur.
- Fonctionnement offline: 100% des actions MVP restent disponibles sans internet cote desktop.
- Sync MVP: une tache creee sur desktop apparait sur mobile apres sync manuelle.
- Sync MVP: une tache supprimee sur un appareil disparait ou est marquee supprimee sur l'autre apres sync.
- Donnees: aucune perte de tache locale pendant une erreur de synchronisation.

## Product Scope

### MVP - Phase 1 Desktop Local

- App desktop Linux avec Tauri.
- Presence dans la barre des taches/system tray.
- Petit panneau ouvert depuis l'icone tray.
- Liste de taches simples dans le panneau.
- Input d'ajout fixe en haut du panneau pour enchainer rapidement les ajouts.
- Ajout par touche `Entree`.
- Suppression via icone poubelle a droite de chaque tache.
- UI complete pour modifier le texte et le statut.
- Stockage local.
- Fonctionnement offline sans compte ni cloud obligatoire.

### Phase 2 - Mobile and Local Sync

- App mobile minimale.
- Synchronisation locale ou USB, manuelle si necessaire.
- Sync reseau local plus fluide.
- Prevention des doublons.
- Propagation des suppressions.
- Feedback de succes ou echec de sync.

### Growth Features (Post-MVP)

- Raccourci clavier global pour ajout rapide.
- Historique ou corbeille simple.
- Packaging Linux propre.
- Amelioration de l'experience mobile.
- Detection et resolution plus claire des conflits de sync.

### Vision (Future)

- Widgets ou raccourcis mobiles.
- Demarrage automatique discret sur desktop.
- Sync plus automatisee sans imposer de cloud.
- Support eventuel d'autres plateformes seulement si le produit Linux est stable.

## User Journeys

### Journey 1 - Capture rapide depuis Linux

Johan travaille sur son poste Linux et pense a une tache courte a ne pas oublier. Il ne veut pas ouvrir une application complete ni interrompre ce qu'il fait. Il clique sur l'icone Taskbar Todolist dans la barre des taches, un petit panneau s'ouvre, le curseur est dans l'input en haut, il saisit une ligne, appuie sur `Entree`, puis peut enchainer une autre tache ou retourner immediatement a son travail.

Le moment de valeur arrive quand la tache est enregistree en quelques secondes, sans changement de contexte. Le produit reussit si l'action ressemble plus a une note instantanee qu'a une gestion de projet.

Capacites revelees:

- icone tray visible et fiable;
- ouverture rapide d'un petit panneau;
- input d'ajout fixe en haut du panneau;
- focus automatique sur l'input;
- ajout par touche `Entree`;
- possibilite d'enchainer plusieurs ajouts sans changer de zone;
- persistance locale immediate;
- fermeture ou retour au travail sans friction.

### Journey 2 - Nettoyage rapide d'une tache

Johan voit une tache qui n'est plus utile ou deja faite. Depuis le tray, il ouvre le panneau, identifie la tache, puis clique sur l'icone poubelle a droite de cette tache. L'action doit etre courte et previsible.

Le moment de valeur arrive quand la liste redevient propre sans effort. Le risque principal est la suppression accidentelle; le MVP doit soit rendre l'action claire, soit prevoir une recuperation simple post-MVP.

Capacites revelees:

- liste lisible dans le tray;
- icone poubelle a droite de chaque tache;
- confirmation legere ou interaction non ambigue;
- stockage local mis a jour immediatement;
- propagation de suppression lors de la sync.

### Journey 3 - Modification dans l'UI complete

Johan ajoute rapidement une tache, puis realise plus tard que son texte n'est pas assez clair. Il ouvre l'UI complete depuis le tray, selectionne la tache, modifie son texte ou son statut, puis ferme la fenetre.

Le moment de valeur arrive quand la barre des taches reste legere, mais qu'une interface plus confortable existe quand une modification demande plus de place.

Capacites revelees:

- ouverture de l'UI complete depuis le tray;
- selection d'une tache;
- edition du texte;
- changement de statut simple;
- sauvegarde locale;
- retour a l'etat tray apres modification.

### Journey 4 - Retrouver ses taches sur mobile

Johan quitte son PC et veut retrouver ses taches sur mobile. Il declenche une synchronisation locale ou USB entre desktop et mobile. Les taches creees sur desktop apparaissent sur mobile, et les suppressions/modifications sont respectees.

Le moment de valeur arrive quand le mobile devient un prolongement du desktop sans necessiter de compte cloud. Le risque principal est la confusion si une tache existe differemment sur les deux appareils.

Capacites revelees:

- stockage local mobile;
- protocole de sync local ou USB;
- identifiants de taches stables;
- gestion des suppressions;
- prevention des doublons;
- feedback de succes ou echec de sync.

### Journey 5 - Recuperation apres erreur de sync

Johan tente de synchroniser, mais le mobile n'est pas connecte correctement ou la sync echoue. L'app doit garder les donnees locales intactes, afficher une erreur comprehensible, et permettre de reessayer.

Le moment de valeur arrive quand l'echec est non destructif. Une sync ratee ne doit jamais faire perdre une tache.

Capacites revelees:

- erreurs de sync visibles;
- retry manuel;
- aucune suppression destructive pendant l'echec;
- etat local fiable;
- message comprehensible;
- journal ou diagnostic simple post-MVP.

### Journey Requirements Summary

Les parcours montrent que le produit doit prioriser cinq capacites:

1. Tray desktop rapide et fiable.
2. Stockage local robuste.
3. UI complete uniquement pour les modifications.
4. Sync locale/USB non destructive.
5. Feedback clair en cas d'erreur.

Le MVP doit eviter toute complexite qui ralentit ces parcours: pas de projets, pas de tags, pas de compte obligatoire, pas de cloud impose.

## Desktop App Specific Requirements

### Project-Type Overview

Taskbar Todolist est une application desktop Linux construite avec Tauri. Le desktop est la surface principale du produit: il fournit l'acces rapide depuis la barre des taches/system tray, le stockage local, l'UI complete de modification et le point de depart de la synchronisation avec le mobile.

L'app mobile existe comme compagnon synchronise, mais le projet est d'abord pilote par l'experience desktop Linux.

### Platform Support

- La plateforme prioritaire du MVP est Linux.
- Le support Windows/macOS est hors scope MVP.
- L'application doit fonctionner dans un environnement desktop Linux avec support system tray.
- Le packaging Linux doit etre prevu post-MVP si le prototype est valide.

### System Integration

- L'application doit exposer une icone dans la barre des taches/system tray.
- Le clic sur l'icone tray doit ouvrir un petit panneau.
- Le panneau tray doit afficher la liste des taches.
- Le panneau tray doit placer l'input d'ajout en haut pour faciliter les ajouts successifs.
- L'input du panneau tray doit ajouter la tache avec la touche `Entree`.
- Chaque tache du panneau tray doit afficher une icone poubelle a droite pour la suppression.
- Le tray doit permettre d'ouvrir l'UI complete.
- Le demarrage automatique au login est une fonctionnalite post-MVP.
- Un raccourci clavier global pour ajout rapide est post-MVP.

### Offline Capabilities

- L'app desktop doit fonctionner sans connexion internet.
- Les actions MVP doivent etre disponibles offline: lire, ajouter, supprimer, modifier.
- Les donnees doivent etre persistees localement immediatement.
- La sync ne doit jamais etre requise pour utiliser l'app desktop.

### Update Strategy

- Le MVP ne necessite pas d'auto-update.
- Les mises a jour peuvent etre manuelles pendant le developpement personnel.
- Une strategie d'update propre peut etre etudiee apres stabilisation du produit Linux.

### Technical Architecture Considerations

- Tauri est la technologie desktop retenue.
- La couche frontend sert l'UI tray/panneau et l'UI complete.
- La couche native Tauri/Rust gere l'integration systeme, le stockage local et les operations de sync.
- Le stockage local doit etre suffisamment robuste pour eviter toute perte de tache.
- SQLite est recommande pour structurer les taches et preparer la sync.
- Le modele de tache doit rester minimal: identifiant, texte, statut, dates de creation/modification, suppression eventuelle.

### Implementation Considerations

- Le prototype doit d'abord valider la faisabilite du tray Linux avec Tauri.
- Les fonctionnalites desktop locales passent avant la sync mobile.
- La sync locale/USB doit etre concue comme une operation non destructive.
- Les erreurs de sync doivent etre visibles et permettre un retry manuel.
- Les fonctionnalites avancees qui alourdissent l'experience doivent rester hors scope MVP.

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach:** Experience MVP. Le but est de valider que l'experience desktop Linux taskbar-first est reellement plus rapide qu'une todolist classique ou une note rapide.

**Resource Requirements:** Une personne capable de travailler sur Tauri, frontend, stockage local, et integration Linux. Le mobile et la sync peuvent venir apres validation du desktop local.

### MVP Feature Set (Phase 1)

**Core User Journeys Supported:**

- Capture rapide depuis Linux.
- Nettoyage rapide d'une tache.
- Modification dans l'UI complete.
- Usage offline local.

**Must-Have Capabilities:**

- App desktop Linux avec Tauri.
- Icone system tray.
- Petit panneau rapide depuis le tray.
- Liste de taches simples.
- Input d'ajout en haut du panneau.
- Ajouts successifs possibles sans deplacer le focus hors de l'input.
- Ajout avec `Entree`.
- Suppression par icone poubelle a droite de chaque tache.
- UI complete pour modifier texte et statut.
- Stockage local persistant.
- Fonctionnement sans internet.

### Post-MVP Features

**Phase 2 - Sync locale et mobile minimal:**

- App mobile minimale.
- Stockage local mobile.
- Sync manuelle locale ou USB.
- Prevention des doublons.
- Propagation des suppressions.
- Feedback succes/echec de sync.

**Phase 3 - Confort et robustesse:**

- Raccourci clavier global pour ajout rapide.
- Demarrage automatique discret.
- Packaging Linux propre.
- Retry sync plus clair.
- Historique ou recuperation simple apres suppression.
- Meilleure resolution des conflits de sync.

### Risk Mitigation Strategy

**Technical Risks:** Le plus gros risque initial est la faisabilite et la fiabilite du system tray Linux avec Tauri. Mitigation: prototyper le tray avant toute complexite mobile ou sync.

**Market/User Risks:** Le produit est personnel; le risque principal est de construire trop lourd pour un besoin simple. Mitigation: garder le MVP centre sur add/delete/edit local.

**Resource Risks:** Une seule personne maintient le projet. Mitigation: limiter le MVP au desktop local et reporter mobile/sync apres validation.

## Functional Requirements

### Task Management

- FR1: L'utilisateur peut creer une tache simple avec un texte.
- FR2: L'utilisateur peut consulter la liste des taches actives.
- FR3: L'utilisateur peut supprimer une tache.
- FR4: L'utilisateur peut modifier le texte d'une tache.
- FR5: L'utilisateur peut changer le statut d'une tache entre a faire et terminee.
- FR6: Le systeme peut conserver les taches supprimees sous une forme permettant leur propagation future en sync.
- FR7: Le systeme peut associer a chaque tache un identifiant stable.
- FR8: Le systeme peut enregistrer les dates de creation et de modification d'une tache.

### Taskbar / Tray Experience

- FR9: L'utilisateur peut acceder a Taskbar Todolist depuis la barre des taches Linux.
- FR10: L'utilisateur peut ouvrir un petit panneau en cliquant sur l'icone tray.
- FR11: L'utilisateur peut saisir une tache dans un input situe en haut du panneau rapide.
- FR12: L'utilisateur peut ajouter la tache saisie avec la touche `Entree` tout en gardant le focus dans l'input pour un ajout suivant.
- FR13: L'utilisateur peut consulter les taches actives depuis le panneau rapide.
- FR14: L'utilisateur peut supprimer une tache depuis le panneau rapide via une icone poubelle situee a droite de cette tache.
- FR15: L'utilisateur peut ouvrir l'UI complete depuis le tray.
- FR16: Le systeme peut rester disponible en arriere-plan via le tray.

### Full Editing UI

- FR17: L'utilisateur peut ouvrir une interface complete de gestion des taches.
- FR18: L'utilisateur peut selectionner une tache dans l'interface complete.
- FR19: L'utilisateur peut modifier une tache depuis l'interface complete.
- FR20: L'utilisateur peut changer le statut d'une tache depuis l'interface complete.
- FR21: L'utilisateur peut fermer l'interface complete tout en gardant l'app disponible dans le tray.

### Local Storage

- FR22: Le systeme peut persister les taches localement sur le desktop.
- FR23: Le systeme peut charger les taches locales au demarrage.
- FR24: Le systeme peut sauvegarder les changements de tache sans connexion internet.
- FR25: Le systeme peut preserver les donnees locales en cas d'echec de synchronisation.
- FR26: Le systeme peut representer localement les suppressions pour eviter les incoherences futures.

### Mobile Companion

- FR27: L'utilisateur peut consulter ses taches sur mobile.
- FR28: L'utilisateur peut creer une tache sur mobile.
- FR29: L'utilisateur peut modifier une tache sur mobile.
- FR30: L'utilisateur peut supprimer une tache sur mobile.
- FR31: Le systeme mobile peut stocker les taches localement.
- FR32: Le systeme mobile peut fonctionner sans connexion internet pour les actions de base.

### Synchronization

- FR33: L'utilisateur peut declencher une synchronisation entre desktop et mobile.
- FR34: Le systeme peut transferer les taches creees sur desktop vers mobile.
- FR35: Le systeme peut transferer les taches creees sur mobile vers desktop.
- FR36: Le systeme peut propager les modifications de taches entre appareils.
- FR37: Le systeme peut propager les suppressions de taches entre appareils.
- FR38: Le systeme peut eviter la creation de doublons pendant la synchronisation.
- FR39: Le systeme peut signaler a l'utilisateur si une synchronisation reussit.
- FR40: Le systeme peut signaler a l'utilisateur si une synchronisation echoue.
- FR41: L'utilisateur peut relancer une synchronisation apres echec.

### Settings and Control

- FR42: L'utilisateur peut acceder a une zone de configuration minimale.
- FR43: L'utilisateur peut voir ou choisir le mode de synchronisation disponible.
- FR44: L'utilisateur peut quitter completement l'application desktop.
- FR45: L'utilisateur peut identifier l'etat general de l'application depuis le tray.

## Non-Functional Requirements

### Performance

- NFR1: Le panneau tray doit s'ouvrir en moins de 1 seconde dans les conditions normales d'utilisation.
- NFR2: L'ajout d'une tache depuis le tray doit etre realisable en moins de 5 secondes cote utilisateur.
- NFR3: La suppression d'une tache depuis le tray doit etre possible par ouverture du panneau puis clic sur l'icone poubelle de la tache.
- NFR4: Le chargement de la liste locale doit se terminer en moins de 500 ms pour 500 taches stockees localement, mesure sur une machine Linux de developpement standard.
- NFR5: En arriere-plan sans sync active, l'app desktop doit rester sous 150 MB de memoire resident set size et sous 2% CPU moyen sur 5 minutes, mesure avec les outils systeme Linux.

### Reliability and Data Safety

- NFR6: Apres creation, modification ou suppression d'une tache, le changement doit etre present apres fermeture et reouverture de l'application dans 100% des tests manuels du scenario MVP.
- NFR7: Une erreur de synchronisation ne doit jamais supprimer ou corrompre les donnees locales.
- NFR8: Les suppressions doivent etre representees de maniere a pouvoir etre synchronisees correctement plus tard.
- NFR9: Le systeme doit permettre de relancer une operation de sync echouee.
- NFR10: Apres redemarrage du systeme, l'application doit charger les taches locales existantes sans erreur dans 100% des tests de redemarrage MVP.

### Security and Privacy

- NFR11: Le MVP ne doit pas necessiter de compte utilisateur.
- NFR12: Le MVP ne doit pas envoyer de donnees vers un cloud par defaut.
- NFR13: Les donnees de taches doivent rester stockees localement sauf action explicite de synchronisation.
- NFR14: Toute synchronisation doit etre declenchee ou acceptee explicitement dans le MVP.

### Linux Compatibility

- NFR15: L'app desktop doit fonctionner sur Linux avec Tauri.
- NFR16: Avant d'implementer mobile ou sync, un prototype Tauri doit demontrer sur Linux l'affichage de l'icone tray, l'ouverture du petit panneau, l'input en haut, l'ajout par `Entree`, la poubelle par tache, et la fermeture du panneau sans quitter le processus.
- NFR17: Le MVP doit prioriser un environnement Linux desktop compatible system tray.
- NFR18: Le support Windows/macOS est hors exigence MVP.

### Usability

- NFR19: Depuis le tray, l'utilisateur doit pouvoir consulter la liste apres un clic sur l'icone, ajouter via l'input du haut + `Entree`, et supprimer via la poubelle a droite de chaque tache.
- NFR20: L'UI complete ne doit pas etre necessaire pour ajouter ou supprimer une tache.
- NFR21: Le produit doit eviter les concepts avances qui compliquent l'usage: projets, tags, priorites complexes, collaboration.
- NFR22: Toute erreur de sync Phase 2 doit afficher un message indiquant l'echec, la cause probable si connue, et au moins une action de recuperation disponible, comme reessayer ou verifier la connexion.

### Maintainability

- NFR23: Le MVP Phase 1 doit pouvoir etre implemente et teste sans backend, sans app mobile et sans service externe obligatoire.
- NFR24: Aucune fonctionnalite Phase 2 ne doit etre necessaire pour satisfaire les FR1-FR26 et FR42-FR45 de la Phase 1.
- NFR25: Le modele de tache de Phase 1 doit deja inclure les champs requis pour la Phase 2: identifiant stable, texte, statut, dates de creation/modification, et marqueur de suppression optionnel.
