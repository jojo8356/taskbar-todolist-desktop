---
stepsCompleted:
  - 1
  - 2
  - 3
  - 4
  - 5
  - 6
  - 7
  - 8
  - 9
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/product-brief-taskbar-todolist.md
  - _bmad-output/planning-artifacts/validation-report-prd.md
  - docs/product.md
  - ../taskbar-todolist-mobile/README.md
  - ../taskbar-todolist-mobile/docs/product.md
  - ../taskbar-todolist-org/README.md
  - ../taskbar-todolist-org/docs/architecture.md
  - ../taskbar-todolist-org/docs/product-roadmap.md
  - ../taskbar-todolist-org/docs/sync-model.md
---

## Executive Summary

### Project Vision

Taskbar Todolist est une todolist personnelle Linux-first, centree sur une icone dans la barre des taches. En Phase 1, l'utilisateur clique sur l'icone, un petit panneau s'ouvre, la liste des taches est visible, un input reste en haut, `Entree` ajoute la tache saisie, et une icone poubelle a droite de chaque tache permet de la supprimer. L'UI complete existe seulement pour les modifications plus confortables. Mobile et sync locale/USB arrivent en Phase 2.

### Target Users

L'utilisateur principal est Johan: utilisateur Linux, usage personnel, besoin de noter rapidement des taches courtes, preference pour un outil minimaliste plutot qu'une suite de productivite complete.

Contexte d'usage principal:

- poste desktop Linux;
- pendant une activite en cours;
- besoin de capture rapide;
- faible tolerance a la friction;
- usage offline/local par defaut.

### Key Design Challenges

- Creer un petit panneau tray suffisamment rapide pour que l'ajout soit plus naturel qu'ouvrir une app ou prendre une note.
- Garder la liste lisible dans un panneau compact.
- Placer l'input en haut de facon stable et evidente pour permettre d'ajouter plusieurs taches plus rapidement.
- Rendre la poubelle a droite de chaque tache visible sans rendre l'interface bruyante.
- Separer clairement les actions rapides du tray et les modifications dans l'UI complete.
- Eviter la suppression accidentelle tout en gardant la poubelle accessible.
- Preparer la future sync mobile sans encombrer la Phase 1 desktop local.
- Rendre les erreurs de sync Phase 2 comprehensibles et recuperables.

### Design Opportunities

- Faire du tray la surface principale, pas un raccourci vers une app.
- Optimiser le flux "j'y pense -> c'est note" avec clic sur l'icone, focus automatique dans l'input du haut, validation par `Entree`, et possibilite d'enchainer plusieurs ajouts sans descendre dans le panneau.
- Creer une UI complete calme, utilitaire et dense pour modifier sans transformer le produit en gestionnaire de projet.
- Utiliser une hierarchie visuelle tres simple: taches actives, actions rapides, acces modification.
- Prevoir des maintenant un etat Phase 2 pour sync locale/USB sans compte cloud.

## Core User Experience

### Defining Experience

L'experience centrale est le flux "j'y pense -> c'est note". L'utilisateur clique sur l'icone dans la barre des taches, un petit panneau s'ouvre, le focus est dans l'input en haut, il saisit une tache, appuie sur `Entree`, puis peut soit enchainer une autre tache, soit retourner immediatement a son activite. Le produit reussit si cette action demande moins d'effort mental qu'ouvrir une app de notes ou une todolist classique.

La deuxieme action critique est le nettoyage: consulter rapidement les taches actives et supprimer une tache inutile ou terminee avec l'icone poubelle situee a droite de cette tache, sans passer par l'UI complete.

L'UI complete est une surface secondaire. Elle sert a modifier le texte ou le statut d'une tache quand le panneau tray n'est pas suffisant.

### Platform Strategy

La plateforme prioritaire est Linux desktop avec Tauri. L'experience est principalement souris/clavier, avec un fort besoin de validation clavier pour l'ajout rapide.

Phase 1:

- desktop Linux;
- tray/system tray;
- petit panneau rapide;
- input fixe en haut;
- ajout par `Entree`;
- poubelle a droite de chaque tache;
- UI complete;
- stockage local;
- offline complet.

Phase 2:

- app mobile compagnon;
- synchronisation locale ou USB;
- feedback de sync;
- recuperation apres erreur.

### Effortless Interactions

Les interactions qui doivent demander le moins d'effort possible:

- cliquer sur l'icone dans la barre des taches;
- ouvrir le petit panneau;
- saisir une tache dans l'input fixe en haut avec focus automatique;
- valider avec `Entree`;
- voir les taches actives sans navigation;
- supprimer une tache via la poubelle a droite de la ligne;
- ouvrir l'UI complete seulement quand une modification detaillee est necessaire;
- revenir au travail sans fermer/gerer manuellement l'application.

### Critical Success Moments

- Premier ajout reussi: clic icone tray, saisie dans l'input du haut, `Entree`, tache visible en moins de 5 secondes.
- L'utilisateur constate que la tache est persistee apres fermeture/reouverture.
- L'utilisateur supprime une tache via la poubelle a droite de la ligne sans confusion.
- L'utilisateur modifie une tache dans l'UI complete sans chercher ou aller.
- Phase 2: une tache desktop apparait sur mobile apres sync.
- Phase 2: une erreur de sync n'entraine aucune perte de donnees.

### Experience Principles

1. **Icon-to-panel:** l'icone tray ouvre le produit; elle n'est pas seulement un raccourci.
2. **Top input:** l'input d'ajout reste en haut du panneau pour accelerer l'ajout d'une ou plusieurs taches.
3. **Enter adds:** `Entree` ajoute la tache saisie.
4. **Trash per row:** chaque tache affiche une poubelle a droite pour la suppression.
5. **No project-management drift:** pas de projets, tags, priorites complexes ou ecrans lourds dans le MVP.
6. **Edit elsewhere:** le tray sert aux actions rapides; l'UI complete sert aux modifications.
7. **Local confidence:** l'utilisateur doit sentir que ses taches sont enregistrees localement et ne dependent pas d'internet.
8. **Failure is recoverable:** une erreur de sync Phase 2 doit expliquer quoi faire et ne jamais detruire les donnees.

## Desired Emotional Response

### Primary Emotional Goals

L'utilisateur doit sentir que l'app est immediate, discrete et sous controle. Le clic sur l'icone ouvre exactement ce qu'il faut: un petit panneau, un input en haut pour ajouter vite, une liste de taches dessous, et rien de plus.

L'emotion principale apres ajout est le soulagement. L'utilisateur a pense a une tache, l'a tapee, a appuye sur `Entree`, et peut soit enchainer une autre tache, soit retourner a ce qu'il faisait.

L'emotion principale apres suppression est la clarte. La poubelle a droite rend l'action evidente, locale a la tache, et rapide.

### Emotional Journey Mapping

**Avant l'action:** l'utilisateur ne veut pas ouvrir une application. Il veut juste noter ou supprimer quelque chose.

**Ouverture:** le panneau doit apparaitre comme une extension de l'icone, pas comme une fenetre lourde.

**Ajout:** le focus dans l'input du haut doit donner l'impression que l'app attend deja la tache. `Entree` doit ajouter la tache sans faire perdre le focus, pour permettre des ajouts successifs.

**Suppression:** la poubelle a droite de chaque tache doit indiquer clairement que la suppression concerne cette ligne.

**Apres l'action:** l'utilisateur doit sentir que l'app s'efface mentalement. La tache est ajoutee ou supprimee, point.

### Micro-Emotions

A creer:

- rapidite;
- evidence;
- calme;
- controle;
- confiance;
- legerete.

A eviter:

- chercher ou ajouter;
- chercher ou supprimer;
- avoir peur de supprimer la mauvaise tache;
- avoir l'impression d'ouvrir une app complete;
- voir trop d'options.

### Design Implications

- **Rapidite** -> panneau compact, input en haut, focus automatique, ajout par `Entree`.
- **Ajouts successifs** -> apres `Entree`, l'input reste actif et vide pour la tache suivante.
- **Evidence** -> input toujours en haut, poubelle toujours a droite de chaque tache.
- **Controle** -> suppression par ligne, pas de suppression globale mise en avant.
- **Calme** -> pas de navigation, pas de sidebar, pas de dashboard.
- **Confiance** -> la tache ajoutee apparait immediatement dans la liste.
- **Legerete** -> le panneau doit etre visuellement simple et utilitaire.

### Emotional Design Principles

1. **Une action, un endroit:** ajouter en haut, supprimer a droite de la tache.
2. **Le panneau n'est pas une fenetre:** il doit rester petit, direct et temporaire.
3. **Le clavier doit suffire pour ajouter:** taper puis `Entree`.
4. **Les ajouts doivent pouvoir s'enchainer:** apres ajout, l'input reste pret.
5. **La souris doit suffire pour supprimer:** viser la poubelle de la ligne.
6. **Aucune organisation forcee:** pas de projet, tag ou priorite dans le panneau.
7. **L'app doit disparaitre apres usage:** elle aide puis laisse l'utilisateur revenir a son travail.

## UX Pattern Analysis & Inspiration

### Inspiring Products Analysis

**Raycast**

Raycast montre la valeur d'une interaction courte, clavier-first, qui reduit le changement de contexte. Les avis soulignent sa rapidite, son cote lanceur central, et sa capacite a lancer des actions sans ouvrir plein d'apps. A retenir: ouverture instantanee, focus direct, action depuis une petite surface. A eviter: complexite d'extensions, reglages lourds, Mac-only.

**Todoist Quick Add**

Todoist est apprecie pour la capture rapide, la saisie fluide et la sync fiable. A retenir: l'ajout doit etre l'action la plus optimisee du produit. A eviter pour le MVP: parsing avance, dates, labels, projets, langage naturel complexe.

**TickTick**

TickTick est souvent apprecie pour son interface claire et riche, mais il montre aussi le risque d'un produit qui devient trop complet. A retenir: liste lisible, action rapide, feedback clair. A eviter: calendrier, kanban, priorites, rappels, templates et surcharge fonctionnelle.

**Microsoft To Do**

Microsoft To Do est fort sur la simplicite: listes, taches, sync, usage personnel. A retenir: experience calme et comprehensible. A eviter: dependance compte/cloud et structure trop "app complete".

### Transferable UX Patterns

- Ouverture rapide depuis une petite surface systeme.
- Focus immediat dans le champ principal.
- Ajout clavier avec `Entree`.
- Input en haut pour enchainer plusieurs ajouts.
- Liste directement visible sous l'input.
- Action de suppression au niveau de chaque ligne.
- Peu ou pas de navigation dans le panneau.
- Feedback immediat apres ajout/suppression.

### Anti-Patterns to Avoid

- Transformer le panneau tray en mini-dashboard.
- Ajouter projets, tags, priorites ou calendrier dans le MVP.
- Cacher l'ajout derriere un bouton.
- Faire perdre le focus apres chaque ajout.
- Rendre la suppression trop loin de la tache concernee.
- Imposer un compte ou une sync cloud.
- Ajouter trop de reglages dans le panneau.

### Design Inspiration Strategy

Adopter:

- la rapidite et le focus clavier de Raycast;
- la capture rapide de Todoist;
- la lisibilite simple de Microsoft To Do;
- la clarte visuelle de TickTick, sans sa richesse fonctionnelle.

Adapter:

- le modele "launcher" en version tray Linux;
- le quick add en input haut persistant;
- la liste de taches en surface compacte, sans navigation.

Eviter:

- les fonctions de productivite avancees;
- les workflows multi-ecrans;
- l'organisation projet;
- les patterns cloud-first.

## Design System Foundation

### 1.1 Design System Choice

Le design system retenu est un systeme custom leger base sur Tailwind CSS et des composants minimalistes propres au produit.

Le produit ne doit pas ressembler a une grosse application web ou a un dashboard. La surface principale est un petit panneau tray: input en haut, liste dessous, poubelle a droite de chaque tache. Un design system lourd creerait trop de bruit pour cette experience.

### Rationale for Selection

**Pourquoi custom leger:**

- Le perimetre UI est tres reduit.
- Le panneau doit rester compact, rapide et discret.
- Les composants necessaires sont peu nombreux.
- Les bibliotheques type Material Design ou Ant Design sont trop dashboard/app complete pour ce MVP.
- Tailwind permet de garder un style controle sans construire une grosse couche de composants.

**Contraintes UX:**

- L'input en haut doit etre immediatement visible.
- La liste doit rester lisible dans un petit espace.
- La poubelle doit etre accessible a droite sans dominer visuellement.
- Les etats focus clavier doivent etre tres clairs.
- L'UI complete doit etre utilitaire, mais secondaire.

### Implementation Approach

Composants de base a definir:

- `TrayPanel`
- `TaskInput`
- `TaskRow`
- `IconButton`
- `EmptyState`
- `TaskList`
- `FullEditWindow`
- `StatusMessage`
- `SyncStatus` pour Phase 2

Comportements UI cles:

- focus automatique dans `TaskInput` a l'ouverture du panneau;
- `Entree` ajoute la tache;
- apres ajout, l'input reste actif et vide;
- chaque `TaskRow` affiche une poubelle a droite;
- hover sur une ligne revele ou accentue la poubelle;
- suppression immediate ou avec recuperation legere post-MVP;
- pas de navigation dans le panneau tray.

### Customization Strategy

Style attendu:

- compact;
- calme;
- utilitaire;
- lisible;
- peu colore;
- compatible theme Linux clair/sombre plus tard.

Tokens initiaux:

- rayon faible: `6px` max;
- spacing dense: `4px`, `8px`, `12px`;
- hauteur ligne tache stable;
- input hauteur compacte;
- icones lucide ou equivalent;
- couleur d'accent uniquement pour focus/etat actif;
- rouge uniquement pour danger/suppression.

Le design doit eviter:

- cartes decoratives;
- gros boutons texte;
- dashboard;
- sidebar;
- animations inutiles;
- palette trop expressive.

## 2. Core User Experience

### 2.1 Defining Experience

L'experience definissante est: **cliquer sur l'icone tray, taper une tache dans l'input en haut, appuyer sur `Entree`, voir la tache apparaitre dans la liste dessous, puis pouvoir enchainer immediatement une autre tache.**

Cette interaction doit etre le coeur du produit. Si elle est reussie, Taskbar Todolist tient sa promesse: noter une tache plus vite qu'ouvrir une app de notes ou une todolist complete.

La suppression est le second geste structurant: chaque tache a une poubelle a droite, donc la suppression est directement associee a la ligne concernee.

### 2.2 User Mental Model

L'utilisateur pense en mode capture rapide, pas gestion de projet.

Son modele mental:

- l'icone tray est le point d'acces;
- le panneau est temporaire;
- l'input en haut sert a ajouter;
- `Entree` confirme;
- la liste dessous montre ce qui existe;
- la poubelle a droite supprime la tache de cette ligne;
- l'UI complete est reservee aux modifications.

Ce modele est familier: champ de recherche/commande en haut, resultats/liste dessous, action par ligne a droite. Il ne demande pas d'apprentissage particulier.

### 2.3 Success Criteria

L'experience coeur est reussie si:

- le panneau s'ouvre en moins de 1 seconde;
- le focus est automatiquement dans l'input en haut;
- `Entree` ajoute la tache;
- apres ajout, l'input reste actif et vide;
- la nouvelle tache apparait immediatement dans la liste;
- la poubelle est visible ou clairement disponible a droite de chaque tache;
- l'utilisateur peut ajouter plusieurs taches a la suite sans toucher la souris;
- l'utilisateur peut supprimer une tache sans ouvrir l'UI complete;
- aucune action secondaire ne gene le flux add/delete.

### 2.4 Novel UX Patterns

Le produit n'invente pas un nouveau pattern. Il combine des patterns familiers:

- launcher: petite surface ouverte depuis une icone;
- command input: champ principal en haut;
- list below input: resultats ou elements sous le champ;
- row action: action contextuelle a droite de chaque ligne;
- keyboard confirm: `Entree` valide.

Le twist produit est l'application stricte de ces patterns a une todolist tray Linux ultra-minimale.

### 2.5 Experience Mechanics

**1. Initiation**

L'utilisateur clique sur l'icone dans la barre des taches. Le panneau s'ouvre pres de l'icone ou dans une position stable dependant du systeme.

**2. Input**

Le focus est automatiquement dans l'input en haut. Le placeholder doit indiquer simplement l'action, par exemple `Ajouter une tache`.

**3. Add**

L'utilisateur tape une tache et appuie sur `Entree`.

Si l'input contient du texte:

- la tache est ajoutee;
- elle apparait dans la liste;
- l'input est vide;
- le focus reste dans l'input.

Si l'input est vide:

- `Entree` ne fait rien;
- aucun message lourd n'est affiche.

**4. List**

Les taches actives sont affichees sous l'input. Chaque ligne contient:

- le texte de la tache;
- une icone poubelle a droite.

**5. Delete**

L'utilisateur clique sur la poubelle de la ligne. La tache disparait de la liste active.

Post-MVP, une recuperation legere peut etre ajoutee. Pour le MVP, l'action doit rester directe et locale a la ligne.

**6. Exit**

L'utilisateur clique ailleurs, ferme le panneau, ou retourne a son travail. L'app reste disponible via l'icone tray.

## Visual Design Foundation

### Color System

Le systeme couleur doit rester utilitaire et discret. Le panneau tray ne doit pas attirer l'oeil comme une app complete; il doit etre lisible, rapide et calme.

**Light theme initial:**

- Background: `#F7F7F5`
- Panel surface: `#FFFFFF`
- Border: `#D8D8D2`
- Primary text: `#1F2328`
- Secondary text: `#6B6F76`
- Muted text: `#8A8F98`
- Accent/focus: `#2563EB`
- Hover row: `#F0F2F4`
- Danger/trash: `#DC2626`
- Danger hover background: `#FEE2E2`

**Dark theme later:**

- Background: `#1E1F22`
- Panel surface: `#26272B`
- Border: `#3A3B40`
- Primary text: `#F4F4F5`
- Secondary text: `#B4B7BE`
- Muted text: `#888C95`
- Accent/focus: `#60A5FA`
- Hover row: `#303137`
- Danger/trash: `#F87171`
- Danger hover background: `#3B1F24`

**Color principles:**

- accent only for focus and active states;
- red only for delete/destructive action;
- no decorative gradients;
- no one-note palette;
- keep contrast high for text and icons.

### Typography System

Typography should feel native, compact and clear.

**Recommended stack:**

- `system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`

**Tray panel type scale:**

- Task text: `14px`, line-height `20px`
- Input text: `14px`, line-height `20px`
- Empty state: `13px`, line-height `18px`
- Secondary/status text: `12px`, line-height `16px`

**Full UI type scale:**

- Page title: `20px`, line-height `28px`
- Section title: `16px`, line-height `24px`
- Body/task text: `14px`, line-height `20px`
- Metadata/status: `12px`, line-height `16px`

No oversized headings in the tray. The tray is a tool surface, not a landing page.

### Spacing & Layout Foundation

**Base spacing:**

- `4px` micro spacing;
- `8px` default spacing;
- `12px` section spacing;
- `16px` outer panel padding max.

**Tray panel layout:**

- panel width: approx. `320px` to `380px`;
- max height: approx. `420px`;
- input fixed at top;
- task list below input;
- each task row has stable height;
- trash icon aligned right;
- no sidebar;
- no nested cards;
- no toolbar unless strictly needed.

**Task row layout:**

- text left;
- trash icon button right;
- row height stable, around `36px`;
- hover state changes background subtly;
- trash icon has fixed hit area, around `28px` to `32px`.

### Accessibility Considerations

- Input focus must be visible.
- `Entree` adds task.
- Empty input + `Entree` does nothing.
- Trash button needs accessible label, e.g. `Supprimer la tache`.
- Text contrast should meet WCAG AA where possible.
- Hit targets should remain usable despite compact layout.
- Row hover must not be the only indication of delete availability if discoverability suffers.

## Design Direction Decision

### Design Directions Explored

Six directions ont ete explorees: Compact Native, Soft Floating, Dark Utility, Quiet Delete, Clear Rows, et Empty State. Toutes conservent le meme modele d'interaction: icone tray, petit panneau, input en haut, ajout avec `Entree`, liste dessous, poubelle a droite de chaque tache.

### Chosen Direction

La direction retenue est **Compact Native**.

Elle peut reprendre l'etat vide de la direction **Empty State**: input toujours disponible en haut, message discret `Aucune tache active`, et indication secondaire `Stocke localement`.

### Design Rationale

Compact Native est le meilleur choix parce qu'il ressemble a un utilitaire systeme plutot qu'a une application web. Il soutient l'objectif principal: ajouter et supprimer vite sans attirer l'attention.

Le panneau reste dense, lisible et calme. La poubelle est accessible sans dominer visuellement. L'input en haut permet d'ajouter rapidement plusieurs taches avec `Entree`.

### Implementation Approach

Implementer d'abord le panneau tray avec:

- largeur autour de `320px`;
- input en haut avec focus automatique;
- liste sous l'input;
- lignes de taches hauteur stable;
- poubelle a droite de chaque ligne;
- hover discret sur ligne;
- etat vide minimal;
- theme clair initial;
- theme sombre post-MVP.

# UX Design Specification taskbar-todolist-desktop

**Author:** Johan
**Date:** 2026-05-17

---
