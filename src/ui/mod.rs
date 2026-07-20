//! Slint popup UI and UI-to-service callbacks.
//!
//! The popup is deliberately compact: tasks stay in a scrollable list, completed
//! rows are sorted last by the repository, and the number of visible rows is
//! controlled by normalized YAML settings plus the runtime screen-height limit.

use crate::app::AppState;
use crate::app::settings::{
    AppLanguage, AppSettings, FALLBACK_MAX_VISIBLE_TASKS, MIN_VISIBLE_TASKS,
    normalize_visible_tasks,
};
use crate::app::windows;
use crate::tasks::model::{Task, TaskStatus};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};
use slint::fontique_08::fontique;
use slint::{CloseRequestResponse, ComponentHandle, Model, ModelRc, VecModel};
use std::cell::RefCell;
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
        height: 116px + root.list_height + (root.settings_open ? 94px : 0px);

        in-out property <string> draft_text;
        in-out property <string> language;
        in-out property <int> visible_task_count;
        in-out property <string> visible_task_count_text;
        in-out property <int> visible_task_limit;
        in-out property <string> visible_task_limit_text;
        in-out property <bool> settings_open;
        in-out property <[TaskRow]> tasks;
        in-out property <string> status_text;
        in-out property <string> selected_task_id;
        in-out property <string> edit_text;
        in-out property <string> edit_status;
        in property <string> delete_icon;
        in property <string> settings_icon;
        property <length> list_height: max(34px, root.visible_task_count * 42px);
        property <bool> is_fr: root.language == "fr";
        callback add_task(string);
        callback delete_task(string);
        callback toggle_task(string);
        callback select_task(string);
        callback save_selected_task(string, string, string);
        callback clear_selection();
        callback toggle_settings();
        callback update_language(string);
        callback update_visible_tasks(string);
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
                    text: root.is_fr ? "Tâche" : "Task";
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
                        placeholder-text: root.is_fr ? "Nouvelle tâche" : "New task";
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
                height: root.list_height;
                viewport-width: 280px;
                viewport-height: max(34px, root.tasks.length * 42px);
                vertical-stretch: 0;

                VerticalLayout {
                    width: 260px;
                    spacing: 6px;

                    if root.tasks.length == 0: Text {
                        text: root.is_fr ? "Aucune tâche active" : "No active tasks";
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

            if root.settings_open: Rectangle {
                width: 280px;
                height: 86px;
                background: #f8fafc;
                border-color: #e2e8f0;
                border-width: 1px;
                border-radius: 4px;
                vertical-stretch: 0;

                VerticalLayout {
                    padding: 8px;
                    spacing: 8px;

                    HorizontalLayout {
                        height: 28px;
                        spacing: 8px;

                        Text {
                            text: root.is_fr ? "Langue" : "Language";
                            width: 116px;
                            color: #334155;
                            vertical-alignment: center;
                        }

                        Button {
                            text: "FR";
                            width: 48px;
                            height: 26px;
                            enabled: root.language != "fr";
                            clicked => {
                                root.update_language("fr");
                            }
                        }

                        Button {
                            text: "EN";
                            width: 48px;
                            height: 26px;
                            enabled: root.language != "en";
                            clicked => {
                                root.update_language("en");
                            }
                        }
                    }

                    HorizontalLayout {
                        height: 28px;
                        spacing: 8px;

                        Text {
                            text: root.is_fr ? "Tâches visibles" : "Visible tasks";
                            width: 116px;
                            color: #334155;
                            vertical-alignment: center;
                        }

                        Rectangle {
                            width: 74px;
                            height: 28px;
                            background: #ffffff;
                            border-color: #cbd5e1;
                            border-width: 1px;
                            border-radius: 4px;

                            LineEdit {
                                text <=> root.visible_task_count_text;
                                placeholder-text: root.visible_task_limit_text;
                                width: parent.width;
                                height: parent.height;
                                horizontal-alignment: center;
                                accepted => {
                                    root.update_visible_tasks(root.visible_task_count_text);
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
                spacing: 8px;
                vertical-stretch: 0;

                Rectangle {
                    width: 34px;
                    height: 28px;
                    border-width: 1px;
                    border-color: settings-touch.has_hover ? #cbd5e1 : #e2e8f0;
                    border-radius: 4px;
                    background: root.settings_open ? #e0f2fe : (settings-touch.has_hover ? #f8fafc : #ffffff);

                    Text {
                        text: root.settings_icon;
                        font-family: "lucide";
                        font-size: 16px;
                        width: parent.width;
                        height: parent.height;
                        color: root.settings_open ? #0369a1 : #475569;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }

                    settings-touch := TouchArea {
                        mouse-cursor: pointer;
                        clicked => {
                            root.toggle_settings();
                        }
                    }
                }

                Button {
                    text: root.is_fr ? "Cacher" : "Hide";
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
                    text: root.is_fr ? "Quitter" : "Quit";
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

/// Creates the hidden main popup window and wires every Slint callback to the
/// task and settings services.
pub fn create_main_window(app_state: &AppState) -> Result<MainWindow, slint::PlatformError> {
    tracing::trace!("create_main_window");
    let window = MainWindow::new()?;
    register_lucide_font();
    window.set_delete_icon(char::from(Icon::Trash2).to_string().into());
    window.set_settings_icon(char::from(Icon::Settings).to_string().into());
    window.set_language(app_state.settings.language.ui_code().into());
    window.set_visible_task_count(app_state.settings.visible_tasks);
    window.set_visible_task_count_text(app_state.settings.visible_tasks.to_string().into());
    window.set_visible_task_limit(FALLBACK_MAX_VISIBLE_TASKS);
    window.set_visible_task_limit_text(visible_task_limit_text(FALLBACK_MAX_VISIBLE_TASKS).into());

    let tasks = app_state.tasks.clone();
    let settings = Rc::new(RefCell::new(app_state.settings.clone()));
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

    let settings_window = window.as_weak();
    window.on_toggle_settings(move || {
        tracing::trace!("ui toggle_settings callback");
        if let Some(window) = settings_window.upgrade() {
            window.set_settings_open(!window.get_settings_open());
        }
    });

    let language_settings = settings.clone();
    let language_window = window.as_weak();
    window.on_update_language(move |language| {
        tracing::trace!(language = %language, "ui update_language callback");
        let Some(language) = AppLanguage::from_ui_code(language.as_str()) else {
            tracing::warn!(language = %language, "ignored invalid ui language");
            return;
        };

        let mut next_settings = language_settings.borrow().clone();
        next_settings.language = language;
        let visible_task_limit = language_window
            .upgrade()
            .map(|window| window.get_visible_task_limit())
            .unwrap_or(FALLBACK_MAX_VISIBLE_TASKS);
        if save_settings(&next_settings, visible_task_limit, "update_language") {
            *language_settings.borrow_mut() = next_settings;
            if let Some(window) = language_window.upgrade() {
                window.set_language(language.ui_code().into());
            }
        }
    });

    let visible_tasks_settings = settings.clone();
    let visible_tasks_window = window.as_weak();
    window.on_update_visible_tasks(move |visible_tasks_text| {
        tracing::trace!(visible_tasks_text = %visible_tasks_text, "ui update_visible_tasks callback");
        let current_window = visible_tasks_window.upgrade();
        let previous_visible_tasks = current_window
            .as_ref()
            .map(|window| window.get_visible_task_count())
            .unwrap_or_else(|| visible_tasks_settings.borrow().visible_tasks);
        let visible_task_limit = current_window
            .as_ref()
            .map(|window| window.get_visible_task_limit())
            .unwrap_or(FALLBACK_MAX_VISIBLE_TASKS);
        let Some(visible_tasks) = parse_visible_tasks(visible_tasks_text.as_str()) else {
            tracing::warn!(
                visible_tasks_text = %visible_tasks_text,
                previous_visible_tasks,
                visible_task_limit,
                "ignored invalid visible task count"
            );
            if let Some(window) = current_window {
                window.set_visible_task_count_text(previous_visible_tasks.to_string().into());
            }
            return;
        };

        let mut next_settings = visible_tasks_settings.borrow().clone();
        next_settings.visible_tasks = normalize_visible_tasks(visible_tasks, visible_task_limit);
        if save_settings(&next_settings, visible_task_limit, "update_visible_tasks") {
            *visible_tasks_settings.borrow_mut() = next_settings.clone();
            if let Some(window) = current_window {
                window.set_visible_task_count(next_settings.visible_tasks);
                window.set_visible_task_count_text(next_settings.visible_tasks.to_string().into());
            }
        } else if let Some(window) = current_window {
            window.set_visible_task_count_text(previous_visible_tasks.to_string().into());
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

/// Applies the tray-detected runtime row limit to the UI and persisted settings.
pub fn apply_visible_task_limit(window: slint::Weak<MainWindow>, visible_task_limit: i32) {
    let visible_task_limit = visible_task_limit.max(MIN_VISIBLE_TASKS);
    tracing::trace!(visible_task_limit, "apply_visible_task_limit");
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(window) = window.upgrade() {
            apply_visible_task_limit_now(&window, visible_task_limit);
        }
    });
}

/// Applies a row limit immediately. Call only from the Slint event loop.
pub fn apply_visible_task_limit_now(window: &MainWindow, visible_task_limit: i32) {
    let visible_task_limit = visible_task_limit.max(MIN_VISIBLE_TASKS);
    window.set_visible_task_limit(visible_task_limit);
    window.set_visible_task_limit_text(visible_task_limit_text(visible_task_limit).into());

    let visible_tasks = normalize_visible_tasks(
        window.get_visible_task_count(),
        window.get_visible_task_limit(),
    );
    window.set_visible_task_count(visible_tasks);
    window.set_visible_task_count_text(visible_tasks.to_string().into());

    let settings = AppSettings {
        language: AppLanguage::from_ui_code(window.get_language().as_str())
            .unwrap_or(AppLanguage::Fr),
        visible_tasks,
    };
    let _ = save_settings(&settings, visible_task_limit, "apply_visible_task_limit");
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

fn save_settings(settings: &AppSettings, visible_task_limit: i32, context: &'static str) -> bool {
    if let Err(error) = settings.save_with_limit(visible_task_limit) {
        tracing::error!(error = %error, context, "failed to save app settings");
        false
    } else {
        tracing::debug!(
            language = settings.language.ui_code(),
            visible_tasks = settings.visible_tasks,
            visible_task_limit,
            context,
            "app settings saved"
        );
        true
    }
}

fn visible_task_limit_text(visible_task_limit: i32) -> String {
    format!("1-{}", visible_task_limit.max(MIN_VISIBLE_TASKS))
}

fn parse_visible_tasks(value: &str) -> Option<i32> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        value.parse::<i32>().ok()
    }
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

    #[test]
    fn visible_task_count_accepts_complete_integer_text() {
        assert_eq!(parse_visible_tasks("5"), Some(5));
        assert_eq!(parse_visible_tasks(" 5 "), Some(5));
    }

    #[test]
    fn visible_task_count_rejects_non_integer_text() {
        assert_eq!(parse_visible_tasks(""), None);
        assert_eq!(parse_visible_tasks("abc"), None);
        assert_eq!(parse_visible_tasks("3.5"), None);
        assert_eq!(parse_visible_tasks("3 tasks"), None);
    }
}
