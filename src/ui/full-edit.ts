import { listen } from "@tauri-apps/api/event";
import { createElement, ListTodo, RefreshCw } from "lucide";

import { listTasks, updateTask } from "../state/task-commands";
import { TaskStore } from "../state/task-store";
import type { Task, TaskStatus } from "../types/task";

export function renderFullEdit(target: HTMLElement): void {
  const store = new TaskStore();
  let selectedId: string | null = null;

  const app = document.createElement("main");
  app.className = "full-edit";
  app.setAttribute("aria-label", "Taskbar Todolist full editor");

  const header = document.createElement("header");
  header.className = "full-edit__header";

  const titleGroup = document.createElement("div");
  titleGroup.className = "full-edit__title-group";

  const titleIcon = createElement(ListTodo);
  titleIcon.classList.add("full-edit__icon");
  titleIcon.setAttribute("aria-hidden", "true");

  const titleText = document.createElement("div");
  const title = document.createElement("h1");
  title.textContent = "Taskbar Todolist";
  const subtitle = document.createElement("p");
  subtitle.className = "full-edit__subtitle";
  subtitle.textContent = "Full editor";
  titleText.append(title, subtitle);

  titleGroup.append(titleIcon, titleText);

  const refreshButton = document.createElement("button");
  refreshButton.className = "text-button";
  refreshButton.type = "button";
  refreshButton.title = "Refresh tasks";
  refreshButton.setAttribute("aria-label", "Refresh tasks");
  refreshButton.append(createElement(RefreshCw), document.createTextNode("Refresh"));
  refreshButton.addEventListener("click", () => {
    void loadTasks();
  });

  header.append(titleGroup, refreshButton);

  const content = document.createElement("section");
  content.className = "full-edit__content";

  const listPanel = document.createElement("section");
  listPanel.className = "full-edit__list-panel";
  listPanel.setAttribute("aria-label", "Active tasks");

  const detailPanel = document.createElement("section");
  detailPanel.className = "full-edit__detail-panel";
  detailPanel.setAttribute("aria-label", "Task details");

  const status = document.createElement("p");
  status.className = "full-edit__status";
  status.textContent = "Loading local tasks...";

  app.append(header, content, status);
  content.append(listPanel, detailPanel);
  target.replaceChildren(app);

  void listen("full-edit-opened", () => {
    void loadTasks();
  });

  void loadTasks();

  async function loadTasks(): Promise<void> {
    try {
      status.textContent = "Loading local tasks...";
      store.replaceAll(await listTasks());
      selectFirstAvailableTask();
      renderAll();
      status.textContent = "Local tasks loaded.";
    } catch {
      status.textContent = "Could not load local tasks.";
    }
  }

  function renderAll(): void {
    renderTasks();
    renderDetail();
  }

  function renderTasks(): void {
    const tasks = store.all();

    if (tasks.length === 0) {
      const emptyState = document.createElement("p");
      emptyState.className = "empty-state";
      emptyState.textContent = "No active tasks";
      listPanel.replaceChildren(emptyState);
      return;
    }

    const list = document.createElement("ul");
    list.className = "full-edit-task-list";
    list.setAttribute("aria-label", "Select a task to edit");

    for (const task of tasks) {
      const row = document.createElement("li");

      const button = document.createElement("button");
      button.className = "full-edit-task";
      button.classList.toggle("full-edit-task--selected", task.id === selectedId);
      button.type = "button";
      button.setAttribute("aria-pressed", String(task.id === selectedId));

      const text = document.createElement("span");
      text.className = "full-edit-task__text";
      text.textContent = task.text;

      const badge = document.createElement("span");
      badge.className = "full-edit-task__status";
      badge.textContent = task.status === "done" ? "Done" : "Todo";

      button.append(text, badge);
      button.addEventListener("click", () => {
        selectedId = task.id;
        status.textContent = "Task selected.";
        renderAll();
      });

      row.append(button);
      list.append(row);
    }

    listPanel.replaceChildren(list);
  }

  function renderDetail(): void {
    const task = selectedTask();

    if (!task) {
      const emptyDetail = document.createElement("p");
      emptyDetail.className = "full-edit__empty-detail";
      emptyDetail.textContent = "Select a task to edit its text or status.";
      detailPanel.replaceChildren(emptyDetail);
      return;
    }

    const form = document.createElement("form");
    form.className = "full-edit-form";

    const title = document.createElement("h2");
    title.textContent = "Task details";

    const textField = document.createElement("label");
    textField.className = "full-edit-form__field";

    const textLabel = document.createElement("span");
    textLabel.className = "full-edit-form__label";
    textLabel.textContent = "Text";

    const textInput = document.createElement("textarea");
    textInput.className = "full-edit-form__textarea";
    textInput.rows = 5;
    textInput.value = task.text;
    textInput.setAttribute("aria-label", "Task text");

    textField.append(textLabel, textInput);

    const statusField = document.createElement("label");
    statusField.className = "full-edit-form__field";

    const statusLabel = document.createElement("span");
    statusLabel.className = "full-edit-form__label";
    statusLabel.textContent = "Status";

    const statusSelect = document.createElement("select");
    statusSelect.className = "full-edit-form__select";
    statusSelect.setAttribute("aria-label", "Task status");
    appendStatusOption(statusSelect, "todo", "Todo", task.status);
    appendStatusOption(statusSelect, "done", "Done", task.status);

    statusField.append(statusLabel, statusSelect);

    const actions = document.createElement("div");
    actions.className = "full-edit-form__actions";

    const saveButton = document.createElement("button");
    saveButton.className = "text-button text-button--primary";
    saveButton.type = "submit";
    saveButton.textContent = "Save";

    actions.append(saveButton);
    form.append(title, textField, statusField, actions);

    form.addEventListener("submit", (event) => {
      event.preventDefault();
      void saveTask(task, textInput.value, statusSelect.value as TaskStatus, saveButton);
    });

    detailPanel.replaceChildren(form);
    textInput.focus();
  }

  async function saveTask(
    task: Task,
    nextText: string,
    nextStatus: TaskStatus,
    saveButton: HTMLButtonElement,
  ): Promise<void> {
    const text = nextText.trim();

    if (!text) {
      status.textContent = "Task text cannot be empty.";
      return;
    }

    try {
      saveButton.disabled = true;
      status.textContent = "Saving task...";
      const updated = await updateTask(task.id, { text, status: nextStatus });
      store.update(updated);
      selectedId = updated.id;
      renderAll();
      status.textContent = "Task saved locally.";
    } catch {
      status.textContent = "Could not save task.";
      saveButton.disabled = false;
    }
  }

  function selectedTask(): Task | null {
    return store.all().find((task) => task.id === selectedId) ?? null;
  }

  function selectFirstAvailableTask(): void {
    const tasks = store.all();
    const selectedStillExists = tasks.some((task) => task.id === selectedId);
    selectedId = selectedStillExists ? selectedId : (tasks[0]?.id ?? null);
  }
}

function appendStatusOption(
  select: HTMLSelectElement,
  value: TaskStatus,
  label: string,
  selectedStatus: TaskStatus,
): void {
  const option = document.createElement("option");
  option.value = value;
  option.textContent = label;
  option.selected = value === selectedStatus;
  select.append(option);
}
