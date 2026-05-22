import { listen } from "@tauri-apps/api/event";
import { createElement, ListTodo, Maximize2, Power, X } from "lucide";

import { hideTrayPanel, quitApp, showFullEdit } from "../state/app-commands";
import { createTask, deleteTask, listTasks } from "../state/task-commands";
import { TaskStore } from "../state/task-store";
import type { Task } from "../types/task";
import { renderTaskInput, wireTrayInputFocus } from "./task-input";
import { renderTaskList } from "./task-list";

export function renderTrayPanel(target: HTMLElement): void {
  const panel = createPanel();
  target.replaceChildren(panel.element);
  wireTrayInputFocus();
  void panel.loadTasks();
}

function createPanel(): { element: HTMLElement; loadTasks: () => Promise<void> } {
  const store = new TaskStore();
  const panel = document.createElement("main");
  panel.className = "tray-panel";
  panel.setAttribute("aria-label", "Taskbar Todolist compact panel");

  const header = document.createElement("header");
  header.className = "tray-panel__header";

  const titleGroup = document.createElement("div");
  titleGroup.className = "tray-panel__title-group";

  const titleIcon = createElement(ListTodo);
  titleIcon.classList.add("tray-panel__icon");
  titleIcon.setAttribute("aria-hidden", "true");

  const title = document.createElement("h1");
  title.textContent = "Tasks";

  titleGroup.append(titleIcon, title);

  const hideButton = document.createElement("button");
  hideButton.className = "icon-button";
  hideButton.type = "button";
  hideButton.title = "Hide panel";
  hideButton.setAttribute("aria-label", "Hide panel");
  hideButton.append(createElement(X));
  hideButton.addEventListener("click", () => {
    void hideTrayPanel();
  });

  const headerActions = document.createElement("div");
  headerActions.className = "tray-panel__header-actions";

  const fullEditButton = document.createElement("button");
  fullEditButton.className = "icon-button";
  fullEditButton.type = "button";
  fullEditButton.title = "Open full editor";
  fullEditButton.setAttribute("aria-label", "Open full editor");
  fullEditButton.append(createElement(Maximize2));
  fullEditButton.addEventListener("click", () => {
    void showFullEdit();
  });

  headerActions.append(fullEditButton, hideButton);
  header.append(titleGroup, headerActions);

  const tasksRegion = document.createElement("section");
  tasksRegion.className = "tasks-region";
  tasksRegion.setAttribute("aria-live", "polite");

  const input = renderTaskInput(async (text) => {
    try {
      setStatus("Saving task...");
      const task = await createTask(text);
      store.add(task);
      renderTasks();
      setStatus("Task saved locally.");
    } catch {
      setStatus("Could not save task.");
    }
  });

  const status = document.createElement("section");
  status.className = "status-strip";
  status.setAttribute("aria-label", "Application status");

  const statusText = document.createElement("span");
  statusText.className = "status-strip__text";
  statusText.textContent = "Local desktop mode. Tray process active.";

  const quitButton = document.createElement("button");
  quitButton.className = "text-button text-button--danger";
  quitButton.type = "button";
  quitButton.title = "Quit app";
  quitButton.setAttribute("aria-label", "Quit app completely");
  quitButton.append(createElement(Power), document.createTextNode("Quit"));
  quitButton.addEventListener("click", () => {
    void quitApp();
  });

  status.append(statusText, quitButton);

  panel.append(header, input, status, tasksRegion);

  void listen("tray-panel-opened", () => {
    void loadTasks();
  });

  async function loadTasks(): Promise<void> {
    try {
      setStatus("Loading local tasks...");
      store.replaceAll(await listTasks());
      renderTasks();
      setStatus("Local desktop mode. Tasks loaded.");
    } catch {
      setStatus("Could not load local tasks.");
    }
  }

  function renderTasks(): void {
    tasksRegion.replaceChildren(renderTaskList(store.all(), handleDeleteTask));
  }

  function setStatus(message: string): void {
    statusText.textContent = message;
  }

  async function handleDeleteTask(task: Task): Promise<void> {
    try {
      setStatus("Deleting task...");
      await deleteTask(task.id);
      store.remove(task.id);
      renderTasks();
      setStatus("Task deleted locally.");
    } catch {
      setStatus("Could not delete task.");
    }
  }

  return { element: panel, loadTasks };
}
