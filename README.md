# Taskbar Todolist Desktop

Application desktop pour gerer une todolist depuis la barre des taches/system tray.

## Fonctionnalites cible

- Afficher les taches dans la barre des taches.
- Ajouter rapidement une tache.
- Supprimer rapidement une tache.
- Ouvrir une UI complete pour modifier une tache.
- Synchroniser avec l'application mobile via le backend.

## Stack proposee

- Tauri pour une app desktop legere.
- TypeScript pour partager les types avec le mobile.
- SQLite local pour le mode offline.
- Client sync commun avec le backend.

## Organisation initiale

```text
src/
  tray/       Integration system tray.
  ui/         Interface complete de modification.
  storage/    Persistance locale.
  sync/       Synchronisation desktop.
docs/
scripts/
```

