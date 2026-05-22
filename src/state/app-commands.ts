import { invoke } from "@tauri-apps/api/core";

export async function hideTrayPanel(): Promise<void> {
  await invoke("hide_tray_panel");
}

export async function showFullEdit(): Promise<void> {
  await invoke("show_full_edit");
}

export async function quitApp(): Promise<void> {
  await invoke("quit_app");
}
