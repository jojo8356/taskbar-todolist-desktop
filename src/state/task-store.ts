import type { Task } from "../types/task";

export class TaskStore {
  #tasks: Task[] = [];

  all(): Task[] {
    return [...this.#tasks];
  }

  replaceAll(tasks: Task[]): void {
    this.#tasks = [...tasks];
  }

  add(task: Task): void {
    this.#tasks = [task, ...this.#tasks.filter((item) => item.id !== task.id)];
  }

  remove(id: string): void {
    this.#tasks = this.#tasks.filter((task) => task.id !== id);
  }

  update(task: Task): void {
    this.#tasks = this.#tasks.map((item) => (item.id === task.id ? task : item));
  }
}
