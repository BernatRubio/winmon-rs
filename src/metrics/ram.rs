use crate::metrics::Metric;
use crate::tray::icon::IconGenerator;
use crate::tray::tray::{MetricEnum, TrayIcon};
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::SystemInformation::GlobalMemoryStatusEx;
use windows::Win32::System::SystemInformation::MEMORYSTATUSEX;

impl Metric for Ram {
    fn task(hwnd_raw: isize) {
        let mut tray = TrayIcon::new(None, IconGenerator::new("RAM", 1));
        let hwnd = HWND(hwnd_raw as *mut _);

        tray.create_icon(hwnd, Ram::LABEL, MetricEnum::Ram);

        let mut ram = Ram::default();

        loop {
            ram.sample();
            tray.update_icon(ram.status.dwMemoryLoad as u8);
            sleep(Duration::from_secs(1));
        }
    }

    fn class_name() -> &'static str {
        "ramWindowClass"
    }
}

pub struct Ram {
    pub status: MEMORYSTATUSEX,
}

impl Ram {
    pub const LABEL: &'static str = "RAM";

    pub fn sample(&mut self) {
        unsafe {
            match GlobalMemoryStatusEx(&mut self.status) {
                Ok(_) => (),
                Err(error) => panic!("Memory could not be refreshed: {:?}", error),
            }
        }
    }
}

impl Default for Ram {
    fn default() -> Self {
        let mut status = MEMORYSTATUSEX::default();
        status.dwLength = size_of_val(&status) as u32;
        Ram { status: status }
    }
}
