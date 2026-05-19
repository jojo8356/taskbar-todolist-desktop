import { createElement, ListTodo, Power, X } from "lucide";

import { hideTrayPanel, quitApp } from "../state/app-commands";
import { renderTaskInput, wireTrayInputFocus } from "./task-input";

export function renderTrayPanel(target: HTMLElement): void {
  target.replaceChildren(createPanel());
  wireTrayInputFocus();
}

function createPanel(): HTMLElement {
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

  header.append(titleGroup, hideButton);

  const input = renderTaskInput();

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

  const emptyState = document.createElement("p");
  emptyState.className = "empty-state";
  emptyState.textContent = "Tasks will appear here in the next sprint story.";

  panel.append(header, input, status, emptyState);

  return panel;
}
