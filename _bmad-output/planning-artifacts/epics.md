---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
  - step-03-create-stories
  - step-04-final-validation
workflowType: "epics-and-stories"
status: "complete"
completedAt: "2026-05-19"
implementationSequence:
  - Epic 1
  - Epic 3
  - Epic 2
  - Epic 4
  - Epic 5
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/architecture.md
  - _bmad-output/planning-artifacts/ux-design-specification.md
  - _bmad-output/planning-artifacts/validation-report-prd.md
---

# taskbar-todolist-desktop - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for taskbar-todolist-desktop, decomposing the requirements from the PRD, UX Design, and Architecture requirements into implementable stories.

## Implementation Sequence

Story and epic numbers are stable traceability identifiers. The implementation order is:

1. Epic 1: Linux Tray App Foundation.
2. Epic 3: Local Persistence and Sync-Ready Task Model.
3. Epic 2: Quick Tray Task Capture and Cleanup.
4. Epic 4: Full Task Editing UI.
5. Epic 5: Mobile Companion and Local Sync.

This sequence prevents Epic 2 from creating temporary task behavior before the SQLite model, Tauri command boundary, and task repository exist. Sprint planning must follow this implementation sequence rather than numeric epic order.

## Requirements Inventory

### Functional Requirements

FR1: L'utilisateur peut creer une tache simple avec un texte.
FR2: L'utilisateur peut consulter la liste des taches actives.
FR3: L'utilisateur peut supprimer une tache.
FR4: L'utilisateur peut modifier le texte d'une tache.
FR5: L'utilisateur peut changer le statut d'une tache entre a faire et terminee.
FR6: Le systeme peut conserver les taches supprimees sous une forme permettant leur propagation future en sync.
FR7: Le systeme peut associer a chaque tache un identifiant stable.
FR8: Le systeme peut enregistrer les dates de creation et de modification d'une tache.
FR9: L'utilisateur peut acceder a Taskbar Todolist depuis la barre des taches Linux.
FR10: L'utilisateur peut ouvrir un petit panneau en cliquant sur l'icone tray.
FR11: L'utilisateur peut saisir une tache dans un input situe en haut du panneau rapide.
FR12: L'utilisateur peut ajouter la tache saisie avec la touche `Entree` tout en gardant le focus dans l'input pour un ajout suivant.
FR13: L'utilisateur peut consulter les taches actives depuis le panneau rapide.
FR14: L'utilisateur peut supprimer une tache depuis le panneau rapide via une icone poubelle situee a droite de cette tache.
FR15: L'utilisateur peut ouvrir l'UI complete depuis le tray.
FR16: Le systeme peut rester disponible en arriere-plan via le tray.
FR17: L'utilisateur peut ouvrir une interface complete de gestion des taches.
FR18: L'utilisateur peut selectionner une tache dans l'interface complete.
FR19: L'utilisateur peut modifier une tache depuis l'interface complete.
FR20: L'utilisateur peut changer le statut d'une tache depuis l'interface complete.
FR21: L'utilisateur peut fermer l'interface complete tout en gardant l'app disponible dans le tray.
FR22: Le systeme peut persister les taches localement sur le desktop.
FR23: Le systeme peut charger les taches locales au demarrage.
FR24: Le systeme peut sauvegarder les changements de tache sans connexion internet.
FR25: Le systeme peut preserver les donnees locales en cas d'echec de synchronisation.
FR26: Le systeme peut representer localement les suppressions pour eviter les incoherences futures.
FR27: L'utilisateur peut consulter ses taches sur mobile.
FR28: L'utilisateur peut creer une tache sur mobile.
FR29: L'utilisateur peut modifier une tache sur mobile.
FR30: L'utilisateur peut supprimer une tache sur mobile.
FR31: Le systeme mobile peut stocker les taches localement.
FR32: Le systeme mobile peut fonctionner sans connexion internet pour les actions de base.
FR33: L'utilisateur peut declencher une synchronisation entre desktop et mobile.
FR34: Le systeme peut transferer les taches creees sur desktop vers mobile.
FR35: Le systeme peut transferer les taches creees sur mobile vers desktop.
FR36: Le systeme peut propager les modifications de taches entre appareils.
FR37: Le systeme peut propager les suppressions de taches entre appareils.
FR38: Le systeme peut eviter la creation de doublons pendant la synchronisation.
FR39: Le systeme peut signaler a l'utilisateur si une synchronisation reussit.
FR40: Le systeme peut signaler a l'utilisateur si une synchronisation echoue.
FR41: L'utilisateur peut relancer une synchronisation apres echec.
FR42: L'utilisateur peut acceder a une zone de configuration minimale.
FR43: L'utilisateur peut voir ou choisir le mode de synchronisation disponible.
FR44: L'utilisateur peut quitter completement l'application desktop.
FR45: L'utilisateur peut identifier l'etat general de l'application depuis le tray.

### NonFunctional Requirements

NFR1: Le panneau tray doit s'ouvrir en moins de 1 seconde, mesure du clic sur l'icone tray jusqu'au panneau visible, avec 500 taches stockees localement sur l'environnement Linux cible du MVP.
NFR2: L'ajout d'une tache depuis le tray doit etre realisable en moins de 5 secondes cote utilisateur.
NFR3: La suppression d'une tache depuis le tray doit etre possible par ouverture du panneau puis clic sur l'icone poubelle de la tache.
NFR4: Le chargement de la liste locale doit se terminer en moins de 500 ms pour 500 taches stockees localement, mesure sur une machine Linux de developpement standard.
NFR5: En arriere-plan sans sync active, l'app desktop doit rester sous 150 MB de memoire resident set size et sous 2% CPU moyen sur 5 minutes, mesure avec les outils systeme Linux.
NFR6: Apres creation, modification ou suppression d'une tache, le changement doit etre present apres fermeture et reouverture de l'application dans 100% des tests manuels du scenario MVP.
NFR7: Une erreur de synchronisation simulee par appareil deconnecte, payload invalide ou interruption de transfert ne doit supprimer ni corrompre aucune donnee locale, verifie par comparaison de l'etat des taches avant et apres le test.
NFR8: Les suppressions doivent etre representees de maniere a pouvoir etre synchronisees correctement plus tard.
NFR9: Le systeme doit permettre de relancer une operation de sync echouee.
NFR10: Apres redemarrage du systeme, l'application doit charger les taches locales existantes sans erreur dans 100% des tests de redemarrage MVP.
NFR11: Le MVP ne doit pas necessiter de compte utilisateur.
NFR12: Le MVP ne doit pas envoyer de donnees vers un cloud par defaut.
NFR13: Les donnees de taches doivent rester stockees localement sauf action explicite de synchronisation.
NFR14: Toute synchronisation doit etre declenchee ou acceptee explicitement dans le MVP.
NFR15: L'app desktop doit fonctionner sur l'environnement Linux cible du MVP avec Tauri, valide par lancement de l'app, presence de l'icone tray, ouverture du panneau, ajout, suppression, modification et persistance locale.
NFR16: Avant d'implementer mobile ou sync, un prototype Tauri doit demontrer sur Linux l'affichage de l'icone tray, l'ouverture du petit panneau, l'input en haut, l'ajout par `Entree`, la poubelle par tache, et la fermeture du panneau sans quitter le processus.
NFR17: Le MVP doit prioriser au moins un environnement Linux desktop compatible system tray ou AppIndicator, documente comme environnement de validation principal avant tout support multi-environnement.
NFR18: Le support Windows/macOS est hors exigence MVP.
NFR19: Depuis le tray, l'utilisateur doit pouvoir consulter la liste apres un clic sur l'icone, ajouter via l'input du haut + `Entree`, et supprimer via la poubelle a droite de chaque tache.
NFR20: L'UI complete ne doit pas etre necessaire pour ajouter ou supprimer une tache.
NFR21: Le produit doit eviter les concepts avances qui compliquent l'usage: projets, tags, priorites complexes, collaboration.
NFR22: Toute erreur de sync Phase 2 doit afficher un message indiquant l'echec, la cause probable si connue, et au moins une action de recuperation disponible, comme reessayer ou verifier la connexion.
NFR23: Le MVP Phase 1 doit pouvoir etre implemente et teste sans backend, sans app mobile et sans service externe obligatoire.
NFR24: Aucune fonctionnalite Phase 2 ne doit etre necessaire pour satisfaire les FR1-FR26 et FR42-FR45 de la Phase 1.
NFR25: Le modele de tache de Phase 1 doit deja inclure les champs requis pour la Phase 2: identifiant stable, texte, statut, dates de creation/modification, et marqueur de suppression optionnel.

### Additional Requirements

- Use the official Tauri Vanilla TypeScript starter as the first implementation foundation.
- Use Tauri v2 as the desktop shell.
- Use Vanilla TypeScript frontend modules in Phase 1; do not introduce React, Vue, or Svelte unless Vanilla TypeScript becomes limiting later.
- Use Tailwind CSS v4 after the base Tauri startup is proven.
- Use Rust-side SQLite persistence with SQLx.
- Do not use Prisma, Drizzle, Kysely, MikroORM, TypeORM, or a Node sidecar in Phase 1.
- Expose task persistence through Tauri commands only.
- Validate Linux tray/AppIndicator behavior before implementing full persistence, mobile, or sync.
- Keep Phase 1 offline-only, backend-free, cloud-free, account-free, and sync-free.
- Use a sync-ready task model from Phase 1: stable ID, text, status, created/updated timestamps, and deletion marker.
- Use soft-delete/tombstone semantics internally through `deleted_at`; active task lists must filter deleted tasks.
- Run SQLx migrations at app startup before task commands become available.
- Keep SQLite access inside `src-tauri`.
- Keep frontend task operations behind `src/state/task-commands.ts`; UI modules must not call raw `invoke(...)` everywhere.
- Keep tray panel and full edit UI backed by the same task command/store boundary.
- Use ISO 8601 UTC strings across Rust, TypeScript, and SQLite text columns.
- Use snake_case for database/Rust command names and camelCase for TypeScript UI fields.
- Any schema change must include a migration and update Rust/TypeScript task types.
- Any user-facing error must map from a stable error code.
- The first implementation priority is to scaffold Tauri and prove tray lifecycle: tray icon appears, click opens panel/window, input can autofocus, panel close does not quit process, explicit quit works.
- Name and document one concrete Linux tray validation baseline before implementation stories begin.

### UX Design Requirements

UX-DR1: Implement the Compact Native direction as the initial visual direction.
UX-DR2: Implement a compact tray panel around 320px to 380px wide and up to about 420px high.
UX-DR3: Place the task input fixed at the top of the tray panel.
UX-DR4: Autofocus the task input when the tray panel opens.
UX-DR5: Add the typed task with `Entree`/Enter.
UX-DR6: After a successful add, clear the input and preserve focus for the next task.
UX-DR7: If input is empty and Enter is pressed, do nothing and avoid heavy error messaging.
UX-DR8: Show active tasks directly below the input with no navigation required.
UX-DR9: Render each task row with task text on the left and a trash icon button on the right.
UX-DR10: Keep task row height stable around 36px.
UX-DR11: Give trash buttons a fixed hit area around 28px to 32px.
UX-DR12: Apply subtle row hover background and reveal/accent the trash action without making the UI noisy.
UX-DR13: Ensure row hover is not the only indication of delete availability if discoverability suffers.
UX-DR14: Use a compact native design system with low visual noise, no sidebar, no dashboard, no nested cards, and no decorative cards.
UX-DR15: Use light theme first with background `#F7F7F5`, panel `#FFFFFF`, border `#D8D8D2`, primary text `#1F2328`, accent `#2563EB`, danger `#DC2626`, and danger hover `#FEE2E2`.
UX-DR16: Keep dark theme as later/post-MVP support using the documented dark color tokens.
UX-DR17: Use accent color only for focus/active states and red only for destructive actions.
UX-DR18: Avoid decorative gradients and one-note palettes.
UX-DR19: Use native-feeling typography with `system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`.
UX-DR20: Use tray panel text scale: task/input 14px/20px, empty state 13px/18px, secondary/status 12px/16px.
UX-DR21: Use full UI type scale: page title 20px/28px, section title 16px/24px, body/task 14px/20px, metadata/status 12px/16px.
UX-DR22: Use compact spacing scale: 4px, 8px, 12px, and 16px max outer panel padding.
UX-DR23: Implement visible input focus states.
UX-DR24: Add accessible label to the trash button, e.g. "Supprimer la tache".
UX-DR25: Keep text/icon contrast at WCAG AA where possible.
UX-DR26: Implement an empty state with input still visible, minimal "Aucune tache active" style messaging, and optional local-storage status.
UX-DR27: Implement the reusable UI modules identified by UX: `TrayPanel`, `TaskInput`, `TaskRow`, `IconButton`, `EmptyState`, `TaskList`, `FullEditWindow`, `StatusMessage`, and defer `SyncStatus` to Phase 2.
UX-DR28: Keep the full edit UI utilitarian and secondary; adding and deleting must not require it.
UX-DR29: Do not introduce project, tag, priority, calendar, kanban, collaboration, or heavy settings concepts into the tray panel.
UX-DR30: Preserve the user mental model: tray icon opens temporary panel, input adds, list shows active tasks, trash deletes that row, full UI is only for modifications.

### FR Coverage Map

FR1: Epic 2 - Create simple task from tray flow.
FR2: Epic 2 - View active tasks in tray panel.
FR3: Epic 2 - Delete task from tray row.
FR4: Epic 4 - Edit task text in full UI.
FR5: Epic 4 - Change task status in full UI.
FR6: Epic 3 - Preserve deleted tasks as sync-ready tombstones.
FR7: Epic 3 - Assign stable task IDs.
FR8: Epic 3 - Store creation and update timestamps.
FR9: Epic 1 - Access app from Linux taskbar/system tray.
FR10: Epic 1 - Open compact panel from tray icon.
FR11: Epic 2 - Type task in top input.
FR12: Epic 2 - Add with Enter and preserve focus.
FR13: Epic 2 - View active tasks in quick panel.
FR14: Epic 2 - Delete task via row-level trash icon.
FR15: Epic 4 - Open full UI from tray.
FR16: Epic 1 - Keep app available in background through tray.
FR17: Epic 4 - Open complete task management interface.
FR18: Epic 4 - Select task in full UI.
FR19: Epic 4 - Modify task in full UI.
FR20: Epic 4 - Change status in full UI.
FR21: Epic 4 - Close full UI while app stays in tray.
FR22: Epic 3 - Persist tasks locally.
FR23: Epic 3 - Load local tasks at startup.
FR24: Epic 3 - Save task changes offline.
FR25: Epic 3 - Preserve local data on sync failure.
FR26: Epic 3 - Represent deletions locally for future sync.
FR27: Epic 5 - View tasks on mobile.
FR28: Epic 5 - Create task on mobile.
FR29: Epic 5 - Modify task on mobile.
FR30: Epic 5 - Delete task on mobile.
FR31: Epic 5 - Store tasks locally on mobile.
FR32: Epic 5 - Mobile base actions work offline.
FR33: Epic 5 - Trigger desktop/mobile sync.
FR34: Epic 5 - Transfer desktop-created tasks to mobile.
FR35: Epic 5 - Transfer mobile-created tasks to desktop.
FR36: Epic 5 - Propagate task modifications.
FR37: Epic 5 - Propagate task deletions.
FR38: Epic 5 - Avoid duplicate tasks during sync.
FR39: Epic 5 - Signal sync success.
FR40: Epic 5 - Signal sync failure.
FR41: Epic 5 - Retry failed sync.
FR42: Epic 1 - Access minimal configuration/control.
FR43: Epic 5 - View or choose sync mode.
FR44: Epic 1 - Quit app completely.
FR45: Epic 1 - Identify app state from tray.

### Story Coverage Map

Story 1.1: FR9, NFR15, NFR16, NFR17, NFR18, NFR23, architecture starter/template requirements.
Story 1.2: FR9, FR10, FR16, NFR1, NFR15, NFR16, UX-DR2, UX-DR4.
Story 1.3: FR42, FR44, FR45, NFR5, NFR15.
Story 2.1: FR10, FR13, NFR19, UX-DR1, UX-DR2, UX-DR14, UX-DR15, UX-DR17, UX-DR18, UX-DR19, UX-DR20, UX-DR22, UX-DR23, UX-DR26.
Story 2.2: FR1, FR11, FR12, NFR2, NFR19, UX-DR3, UX-DR4, UX-DR5, UX-DR6, UX-DR7.
Story 2.3: FR2, FR13, UX-DR8, UX-DR10, UX-DR20, UX-DR26, UX-DR30.
Story 2.4: FR3, FR14, NFR3, NFR20, UX-DR9, UX-DR11, UX-DR12, UX-DR13, UX-DR24, UX-DR25.
Story 3.1: FR6, FR7, FR8, FR22, FR26, NFR8, NFR23, NFR24, NFR25, architecture SQLx/SQLite requirements.
Story 3.2: FR1, FR2, FR3, FR4, FR5, FR22, FR24, architecture Tauri command and error-boundary requirements.
Story 3.3: FR22, FR23, FR24, NFR4, NFR6, NFR10.
Story 3.4: FR6, FR25, FR26, NFR5, NFR7, NFR8, architecture soft-delete and data-safety requirements.
Story 4.1: FR15, FR17, FR21, NFR20, UX-DR28.
Story 4.2: FR4, FR18, FR19, NFR6, architecture shared store/command boundary requirements.
Story 4.3: FR5, FR20, NFR21, UX-DR29.
Story 5.1: FR27, FR28, FR29, FR30, FR31, FR32, NFR25.
Story 5.2: FR33, FR43, NFR11, NFR12, NFR13, NFR14.
Story 5.3: FR34, FR35, FR36, FR38.
Story 5.4: FR37, FR39, FR40, FR41, NFR7, NFR9, NFR22.

## Epic List

### Epic 1: Linux Tray App Foundation

Users can launch Taskbar Todolist as a Linux desktop app, see it in the tray, open a compact panel, and keep the app running in the background.

**FRs covered:** FR9, FR10, FR16, FR42, FR44, FR45

### Epic 3: Local Persistence and Sync-Ready Task Model

Users keep their tasks after app restart, and the desktop data model is ready for future sync without implementing sync yet.

**FRs covered:** FR6, FR7, FR8, FR22, FR23, FR24, FR25, FR26

### Epic 2: Quick Tray Task Capture and Cleanup

Users can add tasks from the tray panel with the top input, see active tasks immediately, and delete tasks from the row-level trash action without opening the full UI.

**FRs covered:** FR1, FR2, FR3, FR11, FR12, FR13, FR14

### Epic 4: Full Task Editing UI

Users can open a secondary full UI from the tray to select tasks, edit task text, change status, and close the UI while the app remains available in the tray.

**FRs covered:** FR4, FR5, FR15, FR17, FR18, FR19, FR20, FR21

### Epic 5: Mobile Companion and Local Sync

Users can use a mobile companion, store tasks locally on mobile, and synchronize tasks between desktop and mobile through local/USB sync with clear success/failure feedback.

**FRs covered:** FR27, FR28, FR29, FR30, FR31, FR32, FR33, FR34, FR35, FR36, FR37, FR38, FR39, FR40, FR41, FR43

## Epic 1: Linux Tray App Foundation

Users can launch Taskbar Todolist as a Linux desktop app, see it in the tray, open a compact panel, and keep the app running in the background.

### Story 1.1: Scaffold Tauri App and Linux Validation Baseline

As a desktop user,
I want the app scaffolded with the selected Tauri starter and a named Linux validation baseline,
So that implementation starts from the agreed architecture and proves the target environment first.

**Acceptance Criteria:**

**Given** the repository is ready for implementation
**When** the app is scaffolded
**Then** it uses the official Tauri Vanilla TypeScript starter
**And** the project can run through the Tauri development command.

**Given** the starter has been scaffolded
**When** dependencies are installed
**Then** dependency installation completes successfully
**And** the repository documents the local development command needed to run the Tauri app.

**Given** the architecture requires a Linux tray validation baseline
**When** the scaffold story is completed
**Then** the chosen MVP Linux desktop environment and tray/AppIndicator setup are documented in the repository
**And** the validation baseline is referenced by later tray lifecycle tests.

**Given** Phase 1 must remain desktop-local
**When** dependencies are added
**Then** no backend, cloud service, account provider, mobile framework, sync runtime, Prisma, Drizzle, Kysely, MikroORM, TypeORM, or Node ORM sidecar is introduced.

### Story 1.2: Tray Icon Opens Compact Panel Without Quitting App

As a desktop user,
I want to open Taskbar Todolist from the Linux tray,
So that I can access tasks without opening a full application window.

**Acceptance Criteria:**

**Given** the app is running on the MVP Linux validation baseline
**When** the app starts
**Then** a tray/system indicator icon is visible.

**Given** the tray icon is visible
**When** the user clicks the tray icon
**Then** a compact panel or panel-like window opens.

**Given** the compact panel is open
**When** the user closes or hides the panel
**Then** the app process remains running in the background
**And** the tray icon remains available.

**Given** the compact panel opens
**When** the input is rendered
**Then** the input can receive autofocus.

### Story 1.3: App Tray Controls and Explicit Quit

As a desktop user,
I want minimal tray controls and an explicit quit action,
So that I can understand and control the app lifecycle.

**Acceptance Criteria:**

**Given** the app is running in the tray
**When** the user opens available tray controls
**Then** the app exposes minimal control actions needed for Phase 1.

**Given** the app is running in the background
**When** the user chooses explicit quit
**Then** the app exits completely
**And** the tray icon disappears.

**Given** the app is running normally
**When** the tray state is displayed
**Then** the user can identify the general app state from the tray or compact status UI.

## Epic 2: Quick Tray Task Capture and Cleanup

Users can add tasks from the tray panel with the top input, see active tasks immediately, and delete tasks from the row-level trash action without opening the full UI.

**Implementation order:** Run this epic after Epic 3. Task creation, listing, and deletion must use the shared `task-commands.ts` -> Tauri command -> Rust repository -> SQLite boundary introduced in Epic 3, not temporary frontend-only state.

### Story 2.1: Compact Native Tray Panel UI

As a desktop user,
I want a compact native-feeling tray panel,
So that task capture feels like a lightweight system utility.

**Acceptance Criteria:**

**Given** the tray panel opens
**When** it is rendered
**Then** it follows the Compact Native visual direction
**And** it is approximately 320px to 380px wide with compact spacing.

**Given** the panel renders
**When** no tasks are active
**Then** the input remains visible at the top
**And** a minimal empty state is shown without onboarding-heavy text.

**Given** the panel UI is implemented
**When** visual styles are inspected
**Then** the light theme uses the documented background, panel, border, text, accent, danger, and danger hover tokens
**And** no sidebar, dashboard, nested cards, decorative cards, or decorative gradients are introduced.

**Given** the panel is keyboard accessible
**When** the input receives focus
**Then** the focus state is visible.

### Story 2.2: Add Task From Top Input With Enter

As a desktop user,
I want to type a task in the top input and press Enter,
So that I can capture a task without breaking my workflow.

**Acceptance Criteria:**

**Given** the tray panel opens
**When** the panel is visible
**Then** the task input is fixed at the top
**And** the input is focused automatically.

**Given** the input contains non-empty text
**When** the user presses Enter
**Then** a new simple task is created
**And** the task appears in the active task list.

**Given** the task command boundary from Epic 3 is available
**When** the user adds a task from the tray
**Then** the UI creates the task through `task-commands.ts`
**And** it does not create a frontend-only task record that bypasses persistence.

**Given** a task is added successfully
**When** the add operation completes
**Then** the input is cleared
**And** focus remains in the input for another task.

**Given** the input is empty
**When** the user presses Enter
**Then** no task is created
**And** no heavy error message is displayed.

**Given** the user adds a task from the tray
**When** measured from tray access to visible task
**Then** the flow can be completed in under 5 seconds by the user.

### Story 2.3: Active Task List Rendering

As a desktop user,
I want to see active tasks directly below the input,
So that I can quickly review what is currently pending.

**Acceptance Criteria:**

**Given** active tasks exist
**When** the tray panel opens
**Then** active tasks render directly below the input without navigation.

**Given** active tasks are rendered
**When** the tray panel loads task data
**Then** it reads active tasks through the shared task store backed by Tauri commands.

**Given** a task row is rendered
**When** the task text is long
**Then** the row remains stable and readable within the compact panel.

**Given** the task list is rendered
**When** the list is inspected
**Then** each task row uses stable row height around 36px
**And** task text appears on the left.

**Given** no active tasks exist
**When** the tray panel opens
**Then** the empty state appears below the input
**And** adding remains immediately available.

### Story 2.4: Delete Task From Row Trash Action

As a desktop user,
I want to delete a task from the trash icon on its row,
So that I can clean up my list without opening the full UI.

**Acceptance Criteria:**

**Given** an active task is visible in the tray panel
**When** the row is rendered
**Then** a trash icon button appears on the right side of that row.

**Given** the user clicks the row trash action
**When** the delete operation succeeds
**Then** the task is removed from the active list.

**Given** the user deletes a task from the tray
**When** the delete action is submitted
**Then** deletion goes through the shared task command boundary
**And** the UI updates the active list only after the command succeeds.

**Given** a trash button is rendered
**When** accessibility information is inspected
**Then** it has an accessible label indicating that it deletes the task.

**Given** a task row is hovered or focused
**When** the trash action becomes visually accented
**Then** the UI remains quiet and does not make deletion visually dominant.

**Given** the user deletes a task from the tray
**When** the action is complete
**Then** the full edit UI was not required.

## Epic 3: Local Persistence and Sync-Ready Task Model

Users keep their tasks after app restart, and the desktop data model is ready for future sync without implementing sync yet.

### Story 3.1: SQLx SQLite Task Schema and Migrations

As a desktop user,
I want tasks stored in a local SQLite database,
So that my tasks survive app restarts without any cloud dependency.

**Acceptance Criteria:**

**Given** the app starts
**When** the database is initialized
**Then** SQLx migrations run before task commands are available.

**Given** the task schema is created
**When** the migration is inspected
**Then** the `tasks` table includes `id`, `text`, `status`, `created_at`, `updated_at`, and nullable `deleted_at`.

**Given** the architecture requires Rust-side persistence
**When** the schema and database access are implemented
**Then** SQLite access exists only under `src-tauri`
**And** no frontend TypeScript module accesses SQLite directly.

**Given** Phase 1 excludes Node ORMs
**When** dependencies are inspected
**Then** Prisma, Drizzle, Kysely, MikroORM, TypeORM, and Node ORM sidecars are absent.

**Given** migrations and repository initialization are complete
**When** a command-level or repository-level smoke test creates a task record
**Then** the task can be read back from SQLite with its stable ID, text, status, timestamps, and nullable `deleted_at` fields intact.

### Story 3.2: Rust Task Repository and Tauri Commands

As a desktop user,
I want task operations to be saved through the native app layer,
So that task behavior is reliable and consistent across tray and full UI.

**Acceptance Criteria:**

**Given** SQLx persistence is available
**When** the task repository is implemented
**Then** it supports create, list active, update, and soft delete operations.

**Given** Tauri commands are registered
**When** the frontend invokes task operations
**Then** `create_task`, `list_tasks`, `update_task`, and `delete_task` are available.

**Given** command DTOs cross the Rust/TypeScript boundary
**When** task data is returned to the frontend
**Then** TypeScript receives camelCase fields
**And** Rust/database internals remain snake_case.

**Given** a validation or storage error occurs
**When** a command fails
**Then** the error maps to a stable machine-readable code and compact message.

### Story 3.3: Persist Tray Tasks Across App Restart

As a desktop user,
I want tasks added from the tray to be available after restarting the app,
So that I can trust the app as local storage.

**Acceptance Criteria:**

**Given** the user creates a task from the tray
**When** the command succeeds
**Then** the task is written to SQLite
**And** the UI updates only after command success.

**Given** tasks exist in SQLite
**When** the app starts
**Then** active tasks load from local storage.

**Given** the app is closed and reopened
**When** the tray panel opens
**Then** previously active tasks are visible.

**Given** task loading is measured with 500 locally stored tasks
**When** active tasks are loaded
**Then** local list loading completes in under 500 ms on the development validation machine.

### Story 3.4: Soft Delete and Local Data Safety

As a desktop user,
I want deleted tasks and failed operations handled safely,
So that local data is not lost or corrupted.

**Acceptance Criteria:**

**Given** the user deletes a task
**When** the delete command succeeds
**Then** the task receives a `deleted_at` value
**And** active task queries hide the task.

**Given** a deleted task exists
**When** raw storage is inspected for future sync readiness
**Then** the deleted task record remains available as a tombstone.

**Given** a task mutation fails
**When** the command returns an error
**Then** the frontend task store does not apply the failed mutation as if it succeeded.

**Given** simulated failure conditions such as invalid payload or interrupted operation
**When** local task state is compared before and after the test
**Then** no local task data is deleted or corrupted.

**Given** the app idles without active sync
**When** CPU and memory are measured over 5 minutes
**Then** the app remains under 150 MB RSS and under 2% average CPU on the validation environment.

## Epic 4: Full Task Editing UI

Users can open a secondary full UI from the tray to select tasks, edit task text, change status, and close the UI while the app remains available in the tray.

### Story 4.1: Open Full Edit UI From Tray

As a desktop user,
I want to open a full edit interface from the tray,
So that I can make changes that need more room than the compact panel.

**Acceptance Criteria:**

**Given** the app is available in the tray
**When** the user chooses the full UI action
**Then** the full edit window opens.

**Given** the full edit window is open
**When** the user closes it
**Then** the app remains available in the tray
**And** the tray panel behavior is unchanged.

**Given** the full edit UI is rendered
**When** visual structure is inspected
**Then** it is utilitarian and secondary
**And** it does not replace quick add/delete tray behavior.

### Story 4.2: Select and Edit Task Text

As a desktop user,
I want to select a task and edit its text,
So that I can clarify tasks after quick capture.

**Acceptance Criteria:**

**Given** active tasks exist
**When** the full edit UI opens
**Then** the user can select a task.

**Given** a task is selected
**When** the user edits the text and saves
**Then** the task text is updated through the shared Tauri command boundary.

**Given** the edit command succeeds
**When** the tray panel next renders
**Then** the updated task text is visible there too.

**Given** the edit command fails
**When** the error is returned
**Then** a compact user-facing status message appears
**And** the task store does not apply an unpersisted edit.

### Story 4.3: Change Task Status in Full UI

As a desktop user,
I want to change a task between todo and done,
So that I can track simple completion state without adding project-management complexity.

**Acceptance Criteria:**

**Given** a task is selected in the full edit UI
**When** the user changes its status
**Then** the task status can be saved as `todo` or `done`.

**Given** the status save succeeds
**When** local storage is inspected through app behavior
**Then** the changed status persists after restart.

**Given** the status changes
**When** the tray panel renders active tasks
**Then** the tray uses the same shared task data source as the full UI.

**Given** full UI functionality is implemented
**When** scope is inspected
**Then** no projects, tags, priorities, calendar, kanban, collaboration, or heavy settings concepts are introduced.

## Epic 5: Mobile Companion and Local Sync

Users can use a mobile companion, store tasks locally on mobile, and synchronize tasks between desktop and mobile through local/USB sync with clear success/failure feedback.

### Story 5.1: Mobile Local Task Companion

As a mobile user,
I want to view and manage the same simple task model on mobile,
So that I can carry my task list away from the desktop.

**Acceptance Criteria:**

**Given** the mobile companion is implemented in Phase 2
**When** the user opens it
**Then** the user can view locally stored tasks.

**Given** the mobile user adds a task
**When** the task is saved
**Then** it uses the same sync-ready fields as desktop: stable ID, text, status, created/updated timestamps, and deletion marker support.

**Given** the mobile app has no internet connection
**When** the user creates, edits, deletes, or views tasks
**Then** base task actions work against mobile local storage.

### Story 5.2: Sync Mode Selection and Explicit Trigger

As a user with desktop and mobile devices,
I want to choose or see the available sync mode and trigger sync explicitly,
So that sync remains under my control without mandatory cloud.

**Acceptance Criteria:**

**Given** Phase 2 sync is available
**When** the user opens sync controls
**Then** the user can see or choose the available sync mode.

**Given** a sync mode is available
**When** the user triggers sync
**Then** the sync operation starts only through explicit user action or acceptance.

**Given** sync is implemented
**When** network/cloud behavior is inspected
**Then** no mandatory public cloud account is required.

### Story 5.3: Bidirectional Task Transfer and Merge

As a user with desktop and mobile tasks,
I want tasks created or changed on either device to appear on the other after sync,
So that both devices stay useful without duplicates.

**Acceptance Criteria:**

**Given** a task was created on desktop
**When** sync completes successfully
**Then** the task appears on mobile.

**Given** a task was created on mobile
**When** sync completes successfully
**Then** the task appears on desktop.

**Given** a task was modified on either device
**When** sync completes successfully
**Then** the modification is propagated to the other device.

**Given** the same task ID is encountered during sync
**When** records are merged
**Then** sync avoids creating duplicate tasks.

### Story 5.4: Deletion Propagation, Sync Feedback, and Retry

As a user syncing devices,
I want deletions, failures, and retries handled clearly,
So that sync problems do not destroy my local tasks.

**Acceptance Criteria:**

**Given** a task was deleted on one device
**When** sync completes successfully
**Then** the deletion marker is propagated to the other device
**And** the task disappears from active lists there.

**Given** sync succeeds
**When** the operation finishes
**Then** the app signals sync success to the user.

**Given** sync fails
**When** the operation stops
**Then** the app signals failure with a compact message that includes the likely cause if known and a recovery action.

**Given** sync failed
**When** the user retries
**Then** the sync can be relaunched.

**Given** sync fails due to disconnected device, invalid payload, or interrupted transfer
**When** local task state is compared before and after sync
**Then** no local task data is deleted or corrupted.
