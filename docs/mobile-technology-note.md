# Note technique: mobile Flutter vs Tauri

## Decision

Le compagnon mobile doit etre implemente avec Flutter.

Tauri reste le choix pour l'application desktop Linux. Tauri mobile n'est pas retenu comme voie principale pour la Phase 2, sauf changement d'architecture explicite plus tard.

## Pourquoi Flutter

- Flutter est plus mature pour une app mobile produit: UI mobile, navigation, stockage local, tests, debug Android/iOS, packaging et ecosysteme de plugins.
- Le compagnon mobile est une surface mobile dediee, pas un simple portage de l'UI desktop web.
- Le projet a besoin d'une app mobile simple, locale et fiable, pas d'une base web partagee a tout prix.

## Position de Tauri mobile

Tauri v2 supporte Android et iOS, et il peut convenir pour:

- prototypes mobiles;
- apps internes simples;
- portage d'une UI web existante;
- cas ou le partage TypeScript/Rust desktop-mobile est prioritaire.

Mais pour ce projet, Tauri mobile est plus risque que Flutter car l'ecosysteme mobile est plus jeune, notamment autour du debug, des plugins natifs et des contraintes Android/iOS.

## Consequence architecture

- Desktop: Tauri + Rust + SQLite/SQLx.
- Mobile: Flutter avec stockage local mobile.
- Sync future: contrat de donnees commun entre desktop et mobile.
- Modele commun de tache: `id`, `text`, `status`, `createdAt`, `updatedAt`, `deletedAt`.
- Pas de backend, compte cloud ou sync obligatoire pour le fonctionnement local.

## References

- Tauri mobile prerequisites: https://v2.tauri.app/start/prerequisites/#configure-for-mobile-targets
- Tauri mobile development: https://v2.tauri.app/develop/#developing-your-mobile-application
- Tauri Android CLI: https://v2.tauri.app/reference/cli/#android
- Tauri iOS CLI: https://v2.tauri.app/reference/cli/#ios
