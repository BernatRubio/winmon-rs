use crate::tray::icon::IconGenerator;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::Shell::{
    NIF_ICON, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW, Shell_NotifyIconW,
};

pub struct TrayIcon {
    nid: Option<NOTIFYICONDATAW>,
    icon_generator: IconGenerator,
}

impl TrayIcon {
    pub fn new(nid: Option<NOTIFYICONDATAW>, icon_generator: IconGenerator) -> Self {
        Self {
            nid: nid,
            icon_generator: icon_generator,
        }
    }

    pub fn create_icon(&mut self, hwnd: HWND, label: &'static str, u_id: MetricEnum) {
        let mut nid = NOTIFYICONDATAW::default();

        nid.cbSize = size_of_val(&nid) as u32;
        nid.hWnd = hwnd;
        nid.uID = u_id as u32;
        let tip: Vec<u16> = label.encode_utf16().collect();
        nid.szTip[..tip.len()].copy_from_slice(&tip);
        nid.uFlags = NIF_ICON | NIF_TIP;
        nid.hIcon = self.icon_generator.generate(0).hicon;

        unsafe {
            if !Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
                panic!("Tray icon could not be created: {:?}", GetLastError());
            }
        }

        self.nid = Some(nid);
    }

    pub fn update_icon(&mut self, value: u8) {
        match &mut self.nid {
            Some(nid) => {
                nid.hIcon = self.icon_generator.generate(value).hicon;

                unsafe {
                    if !Shell_NotifyIconW(NIM_MODIFY, nid).as_bool() {
                        eprintln!("Warning: tray icon update failed: {:?}", GetLastError());
                    }
                }
            }
            None => eprintln!("Warning: tried to update tray icon but it didn't exist"),
        }
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        match &self.nid {
            Some(nid) => unsafe {
                if !Shell_NotifyIconW(NIM_DELETE, nid).as_bool() {
                    eprintln!("Warning: tray icon delete failed {:?}", GetLastError());
                }
            },
            None => return,
        }
    }
}

#[repr(u32)]
pub enum MetricEnum {
    Cpu = 0,
    Ram = 1,
}
