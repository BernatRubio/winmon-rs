mod app;
mod metrics;
mod tray;
mod utils;
mod window;

const CMD_RUN: &str = "run";
const CMD_ADD_AUTOSTART: &str = "add-autostart";
const CMD_REMOVE_AUTOSTART: &str = "remove-autostart";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some(CMD_RUN) => crate::app::run(),
        Some(CMD_ADD_AUTOSTART) => crate::app::add_to_autostart(),
        Some(CMD_REMOVE_AUTOSTART) => crate::app::remove_from_autostart(),
        _ => {}
    }
}
