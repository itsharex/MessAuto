use crate::notification::dialog;
use log::{debug, info, warn};
use rust_i18n::t;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn check_full_disk_access() -> bool {
    let home_dir = match env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => return false,
    };

    let db_path = PathBuf::from(&home_dir).join("Library/Messages/chat.db");
    if !db_path.exists() {
        warn!("Messages database file does not exist: {:?}", db_path);
        return false;
    }

    match fs::File::open(&db_path) {
        Ok(_) => {
            log::debug!("Successfully opened Messages database, full disk access granted");
            true
        }
        Err(e) => {
            warn!(
                "Permission check failed - cannot open Messages database: {}",
                e
            );
            false
        }
    }
}

fn open_full_disk_access_settings() {
    let url = "x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles";
    if let Err(e) = Command::new("open").arg(url).status() {
        log::error!("Failed to open Full Disk Access settings: {}", e);
    }
}

pub fn show_permission_dialog() -> bool {
    let title = t!("dialog.permission_request_title");
    let body = t!("dialog.permission_request_body");
    let button_open = t!("dialog.permission_request_button_open");
    let button_later = t!("dialog.permission_request_button_later");

    info!("Showing permission dialog with title: '{}'", title);
    debug!("Dialog content: '{}'", body);
    debug!("Dialog buttons: '{}' / '{}'", button_open, button_later);

    let user_chose_to_open = dialog(&title, &body, &button_open, &button_later);

    if user_chose_to_open {
        info!("User chose to open settings for Full Disk Access.");
        open_full_disk_access_settings();
        true
    } else {
        info!("User dismissed the permission dialog.");
        false
    }
}
