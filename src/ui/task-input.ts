import { listen } from "@tauri-apps/api/event";

const TRAY_INPUT_ID = "tray-task-input";

export function renderTaskInput(onSubmit: (text: string) => Promise<void>): HTMLInputElement {
  const input = document.createElement("input");
  input.id = TRAY_INPUT_ID;
  input.className = "task-input";
  input.type = "text";
  input.autocomplete = "off";
  input.placeholder = "Add a task...";
  input.setAttribute("aria-label", "Add a task");
  input.addEventListener("keydown", (event) => {
    if (event.key !== "Enter" || event.isComposing) {
      return;
    }

    event.preventDefault();
    const text = input.value;

    if (text.trim() === "") {
      return;
    }

    input.disabled = true;
    void onSubmit(text)
      .then(() => {
        input.value = "";
      })
      .finally(() => {
        input.disabled = false;
        input.focus();
      });
  });

  window.queueMicrotask(() => input.focus());

  return input;
}

export function wireTrayInputFocus(): void {
  window.addEventListener("focus", focusTrayInput);
  void listen("tray-panel-opened", focusTrayInput);
}

function focusTrayInput(): void {
  const input = document.getElementById(TRAY_INPUT_ID);

  if (input instanceof HTMLInputElement) {
    input.focus();
    input.select();
  }
}
