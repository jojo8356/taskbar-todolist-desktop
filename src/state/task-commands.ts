import { invoke } from "@tauri-apps/api/core";
import type { Task, TaskStatus } from "../types/task";

export async function createTask(text: string): Promise<Task> {
  return invoke<Task>("create_task", { text });
}

export async function listTasks(): Promise<Task[]> {
  return invoke<Task[]>("list_tasks");
}

export async function updateTask(
  id: string,
  changes: { text?: string; status?: TaskStatus },
): Promise<Task> {
  return invoke<Task>("update_task", { id, ...changes });
}

export async function deleteTask(id: string): Promise<void> {
  await invoke("delete_task", { id });
}
