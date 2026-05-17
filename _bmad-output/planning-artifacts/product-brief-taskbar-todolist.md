---
title: "Product Brief: Taskbar Todolist"
status: "reviewed-draft"
created: "2026-05-17T15:05:26+02:00"
updated: "2026-05-17T15:08:17+02:00"
inputs:
  - "/home/Johan/Documents/taskbar-todolist-desktop/README.md"
  - "/home/Johan/Documents/taskbar-todolist-desktop/docs/product.md"
  - "/home/Johan/Documents/taskbar-todolist-mobile/README.md"
  - "/home/Johan/Documents/taskbar-todolist-org/README.md"
---

# Product Brief: Taskbar Todolist

## Resume executif

Taskbar Todolist est une application personnelle de gestion de taches simples, pensee d'abord pour Linux et construite autour d'une idee: capturer et nettoyer ses taches sans ouvrir une grosse application. L'app desktop est le point d'entree principal. Elle vit dans la barre des taches ou le system tray, permet d'ajouter rapidement une tache, de supprimer une tache terminee ou inutile, et ouvre une interface complete seulement quand une modification est necessaire.

Le produit global comprend deux surfaces: une app desktop Linux et une app mobile synchronisee. L'objectif n'est pas de concurrencer les suites de productivite completes comme Todoist, TickTick ou Microsoft To Do sur la richesse fonctionnelle. L'objectif est de proposer un outil personnel, rapide, local-first, simple a comprendre, et adapte a un usage quotidien sans friction.

La synchronisation doit rester pragmatique. Le MVP doit fonctionner localement sans compte obligatoire, avec une option de sync manuelle entre desktop et mobile via reseau local ou cable USB. Internet n'est pas le coeur du produit; il sert seulement si une option de synchronisation plus pratique devient utile.

## Probleme

Les todolists classiques deviennent souvent trop lourdes pour les petites taches du quotidien. Ajouter une idee demande d'ouvrir une fenetre, choisir une liste, remplir des champs, parfois attendre une sync cloud. Pour une tache simple, cette friction suffit a casser le flux de travail.

Sur desktop, le besoin principal est souvent immediat: "je dois noter ca maintenant" ou "cette tache peut disparaitre". Les apps existantes ont souvent des raccourcis rapides, mais elles restent organisees autour d'une application complete. Pour un usage personnel minimaliste, beaucoup de leurs fonctions deviennent du bruit: projets, labels, equipes, integrations, filtres avances, calendriers complexes.

Le probleme a resoudre est donc precis: permettre a une personne de capturer, consulter et supprimer des taches simples depuis son environnement de travail Linux, puis retrouver ces taches sur mobile sans transformer l'outil en plateforme de productivite.

## Solution

Taskbar Todolist fournit une experience en deux niveaux.

Le premier niveau est l'action rapide depuis la barre des taches Linux. L'utilisateur peut ouvrir un petit panneau, voir ses taches simples, ajouter une ligne, supprimer une tache, puis retourner a son travail. Cette surface doit etre rapide, stable et discrete.

Le second niveau est l'interface complete. Elle sert uniquement quand une tache doit etre modifiee: changer le texte, marquer comme faite, ou ajuster les quelques champs acceptes par le MVP. L'UI complete ne doit pas devenir le centre de gravite du produit; elle existe pour les cas ou la barre des taches ne suffit plus.

L'app mobile sert de prolongement. Elle permet de retrouver les memes taches hors du poste desktop, avec une experience aussi simple: liste, ajout, suppression, modification basique. La synchronisation est locale-first: les donnees existent d'abord sur les appareils, puis sont echangees entre eux quand une connexion locale, USB ou une option reseau est disponible. Pour le MVP, la sync peut etre declenchee explicitement par l'utilisateur plutot que tournee en permanence en arriere-plan.

## Ce qui rend le produit different

Le produit est different parce qu'il assume un perimetre personnel et minimaliste. Il ne cherche pas a couvrir la gestion de projet, la collaboration, les equipes ou les workflows GTD complets. Il cherche a supprimer le temps mort entre "j'y pense" et "c'est note".

Le choix Linux-first est egalement structurant. Le MVP peut se concentrer sur les contraintes reelles d'un environnement Linux: system tray, demarrage discret, stockage local, packaging, compatibilite desktop. Cette focalisation evite de diluer le projet trop tot dans Windows, macOS, iOS, Android et web.

Enfin, la sync locale ou USB change l'esprit du produit. Au lieu d'imposer un compte cloud, le produit peut rester personnel, simple, controle par l'utilisateur, et utilisable meme sans serveur public.

## Utilisateur cible

L'utilisateur principal est Johan: une personne qui travaille sur Linux, veut capturer rapidement des taches personnelles, et prefere une app directe a une suite de productivite complete.

Les besoins principaux sont:

- ajouter une tache sans interrompre son travail;
- supprimer rapidement ce qui n'est plus utile;
- modifier une tache seulement quand c'est necessaire;
- retrouver les taches sur mobile;
- garder le controle local des donnees;
- eviter les fonctions avancees qui compliquent l'usage quotidien.

## Scope MVP

Inclus dans le MVP:

- app desktop Linux;
- presence dans la barre des taches/system tray;
- liste de taches simples;
- ajout rapide d'une tache;
- suppression d'une tache;
- UI complete pour modifier le texte d'une tache;
- statut simple: a faire ou terminee;
- stockage local;
- app mobile minimale;
- synchronisation locale ou USB entre desktop et mobile, meme si elle est manuelle au depart.

Hors scope MVP:

- collaboration multi-utilisateur;
- comptes utilisateurs obligatoires;
- sync cloud publique obligatoire;
- projets, tags, labels, priorites complexes;
- sous-taches;
- commentaires;
- pieces jointes;
- rappels avances;
- calendrier;
- integrations externes;
- support Windows/macOS en premiere version;
- publication SaaS.

## Criteres de succes

Le produit fonctionne si l'utilisateur l'utilise naturellement plusieurs fois par jour sans y penser.

Signaux de succes MVP:

- ajouter une tache depuis le desktop prend moins de quelques secondes;
- supprimer une tache ne demande pas d'ouvrir l'UI complete;
- l'app reste utilisable sans internet;
- une tache creee sur desktop peut etre retrouvee sur mobile apres sync;
- une tache modifiee sur mobile peut revenir sur desktop sans doublon;
- le nombre de champs reste volontairement limite;
- l'app ne devient pas plus lente ou plus complexe que le carnet de notes qu'elle remplace.

## Approche technique de haut niveau

Le desktop sera construit avec Tauri. Ce choix fixe une base legere pour Linux, avec une UI web moderne et une couche native Rust pour l'integration system tray, le demarrage discret, le stockage local et les futures interactions avec la synchronisation. Le stockage local peut commencer simplement avec SQLite ou un fichier structure, tant que le modele reste pret pour la synchronisation.

Le mobile peut etre construit avec React Native / Expo pour avancer vite, avec stockage local et sync manuelle ou semi-automatique.

La synchronisation MVP doit privilegier la robustesse au lieu de la sophistication: export/import local, sync USB, ou reseau local peuvent suffire. Le modele de donnees doit rester minimal: identifiant stable, texte, statut, dates de creation/modification, et indicateur de suppression si necessaire. Une strategie simple doit eviter les doublons lors des echanges entre appareils.

## Vision

A court terme, Taskbar Todolist devient l'outil personnel de capture et suivi de taches le plus rapide possible sur Linux.

A moyen terme, le produit peut devenir une petite suite personnelle local-first: desktop, mobile, sync controlee par l'utilisateur, et eventuellement widgets ou raccourcis mobiles. La vision n'est pas de devenir une plateforme collaborative, mais un outil fiable et discret qui reste proche du systeme d'exploitation et du rythme de travail quotidien.
