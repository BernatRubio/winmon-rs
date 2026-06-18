use crate::metrics::cpu::Cpu;
use crate::metrics::ram::Ram;
use crate::metrics::spawn_metric;
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ, RegCloseKey, RegDeleteValueW, RegOpenKeyExW,
    RegSetValueExW,
};
use windows::core::PCWSTR;

pub fn run() {
    let handles = vec![spawn_metric::<Cpu>(), spawn_metric::<Ram>()];

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn add_to_autostart() {
    let mut lpvaluename: Vec<u16> = r"winmon-rs".encode_utf16().collect();
    lpvaluename.push(0);

    let mut lpsubkey: Vec<u16> = r"Software\Microsoft\Windows\CurrentVersion\Run"
        .encode_utf16()
        .collect();
    lpsubkey.push(0);

    let exe = std::env::current_exe();

    let mut hkey = HKEY::default();

    match exe {
        Ok(path) => {
            let mut lpdata: Vec<u16> = path.to_string_lossy().to_string().encode_utf16().collect();
            lpdata.push(0);

            unsafe {
                match RegOpenKeyExW(
                    HKEY_CURRENT_USER,
                    PCWSTR(lpsubkey.as_ptr()),
                    None,
                    KEY_SET_VALUE,
                    &mut hkey,
                )
                .ok()
                {
                    Ok(_) => (),
                    Err(e) => eprintln!("RegKey could not be opened: {}", e),
                }

                match RegSetValueExW(
                    hkey,
                    PCWSTR(lpvaluename.as_ptr()),
                    None,
                    REG_SZ,
                    Some(std::slice::from_raw_parts(
                        lpdata.as_ptr() as *const u8,
                        lpdata.len() * 2,
                    )),
                )
                .ok()
                {
                    Ok(_) => {
                        let _ = RegCloseKey(hkey);
                    }
                    Err(e) => {
                        eprintln!("RegKey value could not be set: {}", e);
                        let _ = RegCloseKey(hkey);
                    }
                }
            }
        }
        Err(e) => panic!("Error retreiving path of current executable: {}.", e),
    }
}

pub fn remove_from_autostart() {
    unsafe {
        let mut lpsubkey: Vec<u16> = r"Software\Microsoft\Windows\CurrentVersion\Run"
            .encode_utf16()
            .collect();
        lpsubkey.push(0);

        let mut lpvaluename: Vec<u16> = r"winmon-rs".encode_utf16().collect();
        lpvaluename.push(0);

        let mut hkey = HKEY::default();

        match RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(lpsubkey.as_ptr()),
            None,
            KEY_SET_VALUE,
            &mut hkey,
        )
        .ok()
        {
            Ok(_) => (),
            Err(e) => eprintln!("RegKey could not be opened: {}", e),
        }

        match RegDeleteValueW(hkey, PCWSTR(lpvaluename.as_ptr())).ok() {
            Ok(_) => (),
            Err(e) => eprintln!("RegKey value could not be deleted: {}", e),
        }

        let _ = RegCloseKey(hkey);
    }
}
