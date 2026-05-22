import type { Task } from "../types/task";
import { renderTaskRow } from "./task-row";

export function renderTaskList(tasks: Task[], onDelete: (task: Task) => void): HTMLElement {
  if (tasks.length === 0) {
    const emptyState = document.createElement("p");
    emptyState.className = "empty-state";
    emptyState.textContent = "No active tasks";
    return emptyState;
  }

  const list = document.createElement("ul");
  list.className = "task-list";
  list.setAttribute("aria-label", "Active tasks");

  for (const task of tasks) {
    list.append(renderTaskRow(task, onDelete));
  }

  return list;
}
