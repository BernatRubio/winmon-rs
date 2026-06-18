mod app;
mod metrics;
mod tray;
mod utils;
mod window;

const CMD_ADD_AUTOSTART: &str = "add-autostart";
const CMD_REMOVE_AUTOSTART: &str = "remove-autostart";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some(CMD_ADD_AUTOSTART) => match crate::app::add_to_autostart() {
            Ok(_) => (),
            Err(e) => panic!("Failed adding app to autostart: {}", e),
        },
        Some(CMD_REMOVE_AUTOSTART) => match crate::app::remove_from_autostart() {
            Ok(_) => (),
            Err(e) => panic!("Failed removing app from autostart: {}", e),
        },
        _ => crate::app::run(),
    }
}
