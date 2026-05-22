export type TaskStatus = "todo" | "done";

export interface Task {
  id: string;
  text: string;
  status: TaskStatus;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}

export interface AppCommandError {
  code: string;
  message: string;
}
