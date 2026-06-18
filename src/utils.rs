use windows::Win32::Foundation::FILETIME;

pub fn filetime_to_u64(ft: FILETIME) -> u64 {
    (ft.dwHighDateTime as u64) << 32 | ft.dwLowDateTime as u64
}
