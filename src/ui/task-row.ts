import { createElement, Trash2 } from "lucide";
import type { Task } from "../types/task";

export function renderTaskRow(task: Task, onDelete: (task: Task) => void): HTMLElement {
  const row = document.createElement("li");
  row.className = "task-row";
  row.dataset.taskId = task.id;

  const text = document.createElement("span");
  text.className = "task-row__text";
  text.textContent = task.text;

  const deleteButton = document.createElement("button");
  deleteButton.className = "icon-button task-row__delete";
  deleteButton.type = "button";
  deleteButton.title = "Delete task";
  deleteButton.setAttribute("aria-label", `Delete task: ${task.text}`);
  deleteButton.append(createElement(Trash2));
  deleteButton.addEventListener("click", () => onDelete(task));

  row.append(text, deleteButton);

  return row;
}
