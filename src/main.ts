import { getCurrentWindow } from "@tauri-apps/api/window";
import { renderTrayPanel } from "./ui/tray-panel";
import { renderFullEdit } from "./ui/full-edit";
import "./styles.css";

window.addEventListener("DOMContentLoaded", () => {
  if (getCurrentWindow().label === "full-edit") {
    renderFullEdit(document.body);
    return;
  }

  renderTrayPanel(document.body);
});
