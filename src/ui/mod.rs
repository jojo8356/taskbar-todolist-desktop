use crate::app::AppState;
use crate::app::windows;
use crate::tasks::model::{Task, TaskStatus};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};
use slint::fontique_08::fontique;
use slint::{CloseRequestResponse, ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;
use std::sync::Arc;

slint::slint! {
    import { Button, CheckBox, LineEdit, ScrollView } from "std-widgets.slint";

    export struct TaskRow {
        id: string,
        text: string,
        status: string,
    }

    export component MainWindow inherits Window {
        title: "Taskbar Todolist";
        no-frame: true;
        always-on-top: true;
        width: 300px;
        height: 240px;

        in-out property <string> draft_text;
        in-out property <[TaskRow]> tasks;
        in-out property <string> status_text;
        in-out property <string> selected_task_id;
        in-out property <string> edit_text;
        in-out property <string> edit_status;
        in property <string> delete_icon;
        callback add_task(string);
        callback delete_task(string);
        callback toggle_task(string);
        callback select_task(string);
        callback save_selected_task(string, string, string);
        callback clear_selection();
        callback hide_panel();
        callback quit_app();

        VerticalLayout {
            padding: 10px;
            spacing: 8px;

            HorizontalLayout {
                width: 280px;
                height: 32px;
                spacing: 8px;
                vertical-stretch: 0;

                Text {
                    text: "Tâche";
                    width: 42px;
                    height: 32px;
                    color: #334155;
                    vertical-alignment: center;
                }

                Rectangle {
                    horizontal-stretch: 1;
                    height: 32px;
                    background: #ffffff;
                    border-color: #cbd5e1;
                    border-width: 1px;
                    border-radius: 4px;

                    LineEdit {
                        text <=> root.draft_text;
                        placeholder-text: "Nouvelle tâche";
                        width: parent.width;
                        height: parent.height;
                        accepted => {
                            root.add_task(root.draft_text);
                            root.draft_text = "";
                        }
                    }
                }
            }

            ScrollView {
                width: 280px;
                height: 124px;
                viewport-width: 280px;
                viewport-height: max(34px, root.tasks.length * 42px);
                vertical-stretch: 0;

                VerticalLayout {
                    width: 260px;
                    spacing: 6px;

                    if root.tasks.length == 0: Text {
                        text: "No active tasks";
                        width: 260px;
                        color: #64748b;
                        horizontal-alignment: center;
                        vertical-stretch: 0;
                    }

                    for task in root.tasks: HorizontalLayout {
                        width: 260px;
                        height: 34px;
                        spacing: 8px;
                        alignment: center;
                        vertical-stretch: 0;

                        CheckBox {
                            checked: task.status == "done";
                            width: 22px;
                            height: 22px;
                            toggled => {
                                root.toggle_task(task.id);
                            }
                        }

                        task-label := Rectangle {
                            property <bool> editing: root.selected_task_id != "" && root.selected_task_id == task.id;
                            width: 180px;
                            horizontal-stretch: 1;
                            height: 24px;

                            if !task-label.editing: TouchArea {
                                double-clicked => {
                                    root.select_task(task.id);
                                }
                            }

                            if task-label.editing: edit-input := TextInput {
                                text <=> root.edit_text;
                                width: parent.width;
                                height: parent.height;
                                accepted => {
                                    root.save_selected_task(root.selected_task_id, root.edit_text, root.edit_status);
                                }
                                init => {
                                    self.focus();
                                    self.select-all();
                                }
                            }

                            display-text := Text {
                                text: task.text;
                                visible: !task-label.editing;
                                width: parent.width;
                                height: parent.height;
                                color: task.status == "done" ? #64748b : #0f172a;
                                vertical-alignment: center;
                            }

                            if !task-label.editing && task.status == "done": Rectangle {
                                x: 0px;
                                y: parent.height / 2;
                                width: min(display-text.preferred-width, parent.width);
                                height: 1px;
                                background: #64748b;
                            }
                        }

                        Rectangle {
                            width: 34px;
                            height: 30px;
                            border-width: 1px;
                            border-color: delete-touch.has_hover ? #cbd5e1 : #e2e8f0;
                            border-radius: 4px;
                            background: delete-touch.pressed ? #fee2e2 : (delete-touch.has_hover ? #fef2f2 : #ffffff);

                            Text {
                                text: root.delete_icon;
                                font-family: "lucide";
                                font-size: 16px;
                                width: parent.width;
                                height: parent.height;
                                color: delete-touch.has_hover ? #b91c1c : #475569;
                                horizontal-alignment: center;
                                vertical-alignment: center;
                            }

                            delete-touch := TouchArea {
                                mouse-cursor: pointer;
                                clicked => {
                                    root.delete_task(task.id);
                                }
                            }
                        }
                    }
                }
            }

            Rectangle {
                vertical-stretch: 1;
            }

            HorizontalLayout {
                width: 280px;
                height: 32px;
                spacing: 0px;
                vertical-stretch: 0;

                Button {
                    text: "Hide";
                    width: 70px;
                    height: 28px;
                    clicked => {
                        root.hide_panel();
                    }
                }

                Rectangle {
                    horizontal-stretch: 1;
                }

                Button {
                    text: "Quit";
                    width: 70px;
                    height: 28px;
                    clicked => {
                        root.quit_app();
                    }
                }
            }
        }
    }
}

pub fn create_main_window(app_state: &AppState) -> Result<MainWindow, slint::PlatformError> {
    tracing::trace!("create_main_window");
    let window = MainWindow::new()?;
    register_lucide_font();
    window.set_delete_icon(char::from(Icon::Trash2).to_string().into());

    let tasks = app_state.tasks.clone();
    let active_tasks = tasks.list_active_tasks().unwrap_or_else(|error| {
        tracing::error!(error = %error, "failed to load active tasks for window");
        Vec::new()
    });
    let task_rows = to_task_rows(active_tasks);
    log_task_rows("loaded task rows for slint window", &task_rows);
    let task_model = Rc::new(VecModel::from(task_rows));

    window.set_tasks(ModelRc::from(task_model.clone()));
    window.set_status_text(status_text(task_model.row_count()).into());
    tracing::debug!(
        row_count = task_model.row_count(),
        delete_icon = %window.get_delete_icon(),
        "main window model initialized"
    );

    window
        .window()
        .on_close_requested(|| CloseRequestResponse::HideWindow);

    let add_task_model = task_model.clone();
    let add_window = window.as_weak();
    window.on_add_task(move |text| {
        tracing::trace!(text = %text, "ui add_task callback");
        if let Ok(task) = tasks.create_task(text.to_string()) {
            add_task_model.push(to_task_row(task));
            if let Some(window) = add_window.upgrade() {
                window.set_status_text(status_text(add_task_model.row_count()).into());
            }
            log_model_rows("ui model after add_task", &add_task_model);
        } else {
            tracing::warn!(text = %text, "ui add_task callback ignored failed create");
        }
    });

    let delete_tasks = app_state.tasks.clone();
    let delete_task_model = task_model.clone();
    let delete_window = window.as_weak();
    window.on_delete_task(move |id| {
        tracing::trace!(task_id = %id, "ui delete_task callback");
        if delete_tasks.delete_task(id.to_string()).is_ok() {
            replace_tasks(
                &delete_task_model,
                list_active_or_empty(&delete_tasks, "delete_task"),
            );
            if let Some(window) = delete_window.upgrade() {
                window.set_status_text(status_text(delete_task_model.row_count()).into());
                if window.get_selected_task_id() == id {
                    clear_selected_task(&window);
                }
            }
            log_model_rows("ui model after delete_task", &delete_task_model);
        } else {
            tracing::warn!(task_id = %id, "ui delete_task callback failed");
        }
    });

    let toggle_tasks = app_state.tasks.clone();
    let toggle_task_model = task_model.clone();
    window.on_toggle_task(move |id| {
        tracing::trace!(task_id = %id, "ui toggle_task callback");
        if let Some(row) = find_task_row(&toggle_task_model, id.as_str()) {
            let next_status = if row.status == "done" {
                TaskStatus::Todo
            } else {
                TaskStatus::Done
            };
            if toggle_tasks
                .update_task(id.to_string(), None, Some(next_status))
                .is_ok()
            {
                replace_tasks(
                    &toggle_task_model,
                    list_active_or_empty(&toggle_tasks, "toggle_task"),
                );
                log_model_rows("ui model after toggle_task", &toggle_task_model);
            }
        } else {
            tracing::warn!(task_id = %id, "ui toggle_task callback could not find row");
        }
    });

    let select_task_model = task_model.clone();
    let select_window = window.as_weak();
    window.on_select_task(move |id| {
        tracing::trace!(task_id = %id, "ui select_task callback");
        if let (Some(window), Some(row)) = (
            select_window.upgrade(),
            find_task_row(&select_task_model, id.as_str()),
        ) {
            window.set_selected_task_id(row.id);
            window.set_edit_text(row.text);
            window.set_edit_status(row.status);
            tracing::debug!(
                selected_task_id = %window.get_selected_task_id(),
                edit_text = %window.get_edit_text(),
                edit_status = %window.get_edit_status(),
                "inline edit state prepared"
            );
        } else {
            tracing::warn!(task_id = %id, "ui select_task callback could not enter edit mode");
        }
    });

    let save_tasks = app_state.tasks.clone();
    let save_task_model = task_model.clone();
    let save_window = window.as_weak();
    window.on_save_selected_task(move |id, text, status| {
        tracing::trace!(task_id = %id, text = %text, status = %status, "ui save_selected_task callback");
        let Some(status) = TaskStatus::parse(status.as_str()) else {
            tracing::warn!(task_id = %id, status = %status, "invalid task status from UI");
            return;
        };

        if save_tasks
            .update_task(id.to_string(), Some(text.to_string()), Some(status))
            .is_ok()
        {
            replace_tasks(&save_task_model, list_active_or_empty(&save_tasks, "save_selected_task"));
            if let Some(window) = save_window.upgrade() {
                clear_selected_task(&window);
            }
            log_model_rows("ui model after save_selected_task", &save_task_model);
        } else {
            tracing::warn!(task_id = %id, "ui save_selected_task callback failed");
        }
    });

    let clear_window = window.as_weak();
    window.on_clear_selection(move || {
        tracing::trace!("ui clear_selection callback");
        if let Some(window) = clear_window.upgrade() {
            clear_selected_task(&window);
        }
    });

    let weak_window = window.as_weak();
    window.on_hide_panel(move || {
        tracing::trace!("ui hide_panel callback");
        if let Some(window) = weak_window.upgrade() {
            let _ = window.hide();
        }
    });

    window.on_quit_app(|| {
        tracing::trace!("ui quit_app callback");
        let _ = windows::quit_app();
    });

    Ok(window)
}

fn to_task_rows(tasks: Vec<Task>) -> Vec<TaskRow> {
    tasks.into_iter().map(to_task_row).collect()
}

fn to_task_row(task: Task) -> TaskRow {
    TaskRow {
        id: task.id.into(),
        text: task.text.into(),
        status: task.status.as_str().into(),
    }
}

fn replace_tasks(model: &Rc<VecModel<TaskRow>>, tasks: Vec<Task>) {
    let rows = to_task_rows(tasks);
    log_task_rows("replace_tasks", &rows);
    model.set_vec(rows);
}

fn find_task_row(model: &Rc<VecModel<TaskRow>>, id: &str) -> Option<TaskRow> {
    (0..model.row_count())
        .filter_map(|index| model.row_data(index))
        .find(|row| row.id == id)
}

fn status_text(task_count: usize) -> String {
    format!("Local desktop - {task_count} active task(s)")
}

fn clear_selected_task(window: &MainWindow) {
    tracing::trace!(selected_task_id = %window.get_selected_task_id(), "clear_selected_task");
    window.set_selected_task_id("".into());
    window.set_edit_text("".into());
    window.set_edit_status("todo".into());
}

fn register_lucide_font() {
    tracing::trace!("register_lucide_font");
    let blob = fontique::Blob::new(Arc::new(LUCIDE_FONT_BYTES.to_vec()));
    let mut collection = slint::fontique_08::shared_collection();
    collection.register_fonts(blob, None);
}

fn list_active_or_empty(tasks: &crate::tasks::TaskService, context: &str) -> Vec<Task> {
    tasks.list_active_tasks().unwrap_or_else(|error| {
        tracing::error!(error = %error, context, "failed to refresh active tasks");
        Vec::new()
    })
}

fn log_task_rows(message: &'static str, rows: &[TaskRow]) {
    tracing::debug!(count = rows.len(), message);
    for (index, row) in rows.iter().enumerate() {
        tracing::trace!(
            index,
            task_id = %row.id,
            text = %row.text,
            status = %row.status,
            message
        );
    }
}

fn log_model_rows(message: &'static str, model: &Rc<VecModel<TaskRow>>) {
    tracing::debug!(count = model.row_count(), message);
    for index in 0..model.row_count() {
        if let Some(row) = model.row_data(index) {
            tracing::trace!(
                index,
                task_id = %row.id,
                text = %row.text,
                status = %row.status,
                message
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_model_displays_all_active_tasks() {
        let app_state = AppState::in_memory().unwrap();
        let first = app_state
            .tasks
            .create_task("first task".to_string())
            .unwrap();
        let second = app_state
            .tasks
            .create_task("second task".to_string())
            .unwrap();
        let deleted = app_state
            .tasks
            .create_task("deleted task".to_string())
            .unwrap();
        app_state
            .tasks
            .update_task(second.id.clone(), None, Some(TaskStatus::Done))
            .unwrap();
        app_state.tasks.delete_task(deleted.id).unwrap();

        let window = create_main_window(&app_state).unwrap();
        let rows = window.get_tasks();

        assert_eq!(rows.row_count(), 2);
        assert_eq!(rows.row_data(0).unwrap().id, first.id);
        assert_eq!(rows.row_data(0).unwrap().text, "first task");
        assert_eq!(rows.row_data(0).unwrap().status, TaskStatus::Todo.as_str());
        assert_eq!(rows.row_data(1).unwrap().id, second.id);
        assert_eq!(rows.row_data(1).unwrap().text, "second task");
        assert_eq!(rows.row_data(1).unwrap().status, TaskStatus::Done.as_str());
    }
}
