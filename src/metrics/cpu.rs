use crate::metrics::Metric;
use crate::tray::icon::IconGenerator;
use crate::tray::tray::{MetricEnum, TrayIcon};
use crate::utils::filetime_to_u64;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::{FILETIME, HWND};
use windows::Win32::System::Threading::GetSystemTimes;

impl Metric for Cpu {
    fn task(hwnd_raw: isize) {
        let mut tray = TrayIcon::new(None, IconGenerator::new("CPU", 3));
        let hwnd = HWND(hwnd_raw as *mut _);

        tray.create_icon(hwnd, Cpu::LABEL, MetricEnum::Cpu);

        let mut cpu = Cpu::default();

        cpu.sample();

        loop {
            cpu.sample();
            tray.update_icon(cpu.used_percent());
            sleep(Duration::from_secs(1));
        }
    }

    fn class_name() -> &'static str {
        "cpuWindowClass"
    }
}

#[derive(Default)]
pub struct Cpu {
    current: CpuSnapshot,
    previous: CpuSnapshot,
}

impl Cpu {
    pub const LABEL: &'static str = "CPU";

    fn used_percent(&self) -> u8 {
        self.delta().used_percent()
    }

    /// The first call to `sample` does not produce a valid CPU usage value,
    /// because usage is calculated from the difference between two samples over time.
    fn sample(&mut self) {
        let mut snapshot = CpuSnapshot::default();

        unsafe {
            match GetSystemTimes(
                Some(&mut snapshot.idle),
                Some(&mut snapshot.kernel),
                Some(&mut snapshot.user),
            ) {
                Ok(_) => (),
                Err(error) => panic!("System times could not be refreshed: {:?}", error),
            }
        }

        self.previous = std::mem::replace(&mut self.current, snapshot);
    }

    fn delta(&self) -> CpuDelta {
        let idle = filetime_to_u64(self.current.idle) - filetime_to_u64(self.previous.idle);
        let kernel = filetime_to_u64(self.current.kernel) - filetime_to_u64(self.previous.kernel);
        let user = filetime_to_u64(self.current.user) - filetime_to_u64(self.previous.user);

        CpuDelta { idle, kernel, user }
    }
}

#[derive(Default)]
struct CpuSnapshot {
    idle: FILETIME,
    kernel: FILETIME,
    user: FILETIME,
}

struct CpuDelta {
    idle: u64,
    kernel: u64,
    user: u64,
}

impl CpuDelta {
    fn total(&self) -> u64 {
        // kernel also includes idle time (Windows GetSystemTimes behavior)
        self.kernel + self.user
    }
    fn used(&self) -> u64 {
        self.total() - self.idle
    }
    fn used_percent(&self) -> u8 {
        let total = self.total();
        if total == 0 {
            return 0;
        }
        // Percentage: (used / total) * 100
        // Implemented as integer math: used * 100 / total
        // Rounds to nearest integer by adding total / 2 before division
        ((self.used() * 100 + total / 2) / total) as u8
    }
}
