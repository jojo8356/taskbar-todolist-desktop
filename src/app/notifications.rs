use std::env;
use std::process::Command;

const APP_NAME: &str = "Taskbar Todolist";
const ICON_NAME: &str = "task-due";

pub fn show_todolist_initialized(active_task_count: usize) {
    let message = startup_message(desktop_environment(), active_task_count);
    let args = notify_send_args(&message.title, &message.body);

    let _ = Command::new("notify-send").args(args).spawn();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StartupMessage {
    title: String,
    body: String,
}

fn desktop_environment() -> Option<String> {
    env::var("XDG_CURRENT_DESKTOP")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            env::var("DESKTOP_SESSION")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
}

fn startup_message(desktop: Option<String>, active_task_count: usize) -> StartupMessage {
    let task_text = match active_task_count {
        0 => "Aucune tache active".to_owned(),
        1 => "1 tache active".to_owned(),
        count => format!("{count} taches actives"),
    };

    let body = if desktop
        .as_deref()
        .is_some_and(|value| value.to_lowercase().contains("mate"))
    {
        format!("{task_text}. L'app tourne en arriere-plan dans la zone de notification MATE.")
    } else {
        format!("{task_text}. L'app tourne en arriere-plan dans la zone de notification.")
    };

    StartupMessage {
        title: "Todolist initialisee".to_owned(),
        body,
    }
}

fn notify_send_args(title: &str, body: &str) -> Vec<String> {
    [
        "--app-name",
        APP_NAME,
        "--icon",
        ICON_NAME,
        "--urgency",
        "low",
        "--expire-time",
        "4000",
        title,
        body,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_message_mentions_mate_notification_area() {
        let message = startup_message(Some("MATE".to_owned()), 3);

        assert_eq!(message.title, "Todolist initialisee");
        assert!(message.body.contains("3 taches actives"));
        assert!(message.body.contains("zone de notification MATE"));
    }

    #[test]
    fn startup_message_handles_zero_tasks() {
        let message = startup_message(Some("GNOME".to_owned()), 0);

        assert!(message.body.contains("Aucune tache active"));
        assert!(message.body.contains("zone de notification."));
        assert!(!message.body.contains("MATE"));
    }

    #[test]
    fn notify_send_args_use_desktop_notification_options() {
        let args = notify_send_args("Title", "Body");

        assert_eq!(
            args,
            vec![
                "--app-name",
                APP_NAME,
                "--icon",
                ICON_NAME,
                "--urgency",
                "low",
                "--expire-time",
                "4000",
                "Title",
                "Body",
            ]
        );
    }
}
