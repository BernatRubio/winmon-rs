use crate::metrics::cpu::Cpu;
use crate::metrics::ram::Ram;
use crate::metrics::spawn_metric;
use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};

pub fn run() {
    let handles = vec![spawn_metric::<Cpu>(), spawn_metric::<Ram>()];

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn add_to_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run = hkcu.open_subkey_with_flags(
        r"Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )?;

    let exe_path = std::env::current_exe()?;
    run.set_value("winmon-rs", &exe_path.to_string_lossy().to_string())?;

    Ok(())
}

pub fn remove_from_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run =
        hkcu.open_subkey_with_flags(r"Software\Microsoft\Windows\CurrentVersion\Run", KEY_WRITE)?;

    run.delete_value("winmon-rs")?;

    Ok(())
}
