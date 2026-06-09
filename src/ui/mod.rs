use crate::app::AppState;
use slint::{CloseRequestResponse, ComponentHandle};

slint::slint! {
    export component MainWindow inherits Window {
        title: "Taskbar Todolist";
        width: 360px;
        height: 420px;

        in-out property <string> draft_text;
        callback add_task(string);

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
                text: "Ready for Rust native tray validation";
                color: #334155;
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

    Ok(window)
}
