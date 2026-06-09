use crate::app::AppState;
use crate::app::windows;
use slint::{CloseRequestResponse, ComponentHandle};

slint::slint! {
    import { Button } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Taskbar Todolist";
        width: 360px;
        height: 420px;

        in-out property <string> draft_text;
        callback add_task(string);
        callback hide_panel();
        callback quit_app();

        VerticalLayout {
            padding: 10px;
            spacing: 8px;

            TextInput {
                text <=> root.draft_text;
                accepted => {
                    root.add_task(root.draft_text);
                    root.draft_text = "";
                }
            }

            Text {
                text: "Local desktop - tray process active";
                color: #334155;
            }

            HorizontalLayout {
                spacing: 8px;

                Button {
                    text: "Hide";
                    clicked => {
                        root.hide_panel();
                    }
                }

                Button {
                    text: "Quit";
                    clicked => {
                        root.quit_app();
                    }
                }
            }
        }
    }
}

pub fn create_main_window(app_state: &AppState) -> Result<MainWindow, slint::PlatformError> {
    let window = MainWindow::new()?;
    let tasks = app_state.tasks.clone();

    window
        .window()
        .on_close_requested(|| CloseRequestResponse::HideWindow);

    window.on_add_task(move |text| {
        let _ = tasks.create_task(text.to_string());
    });

    let weak_window = window.as_weak();
    window.on_hide_panel(move || {
        if let Some(window) = weak_window.upgrade() {
            let _ = window.hide();
        }
    });

    window.on_quit_app(|| {
        let _ = windows::quit_app();
    });

    Ok(window)
}
