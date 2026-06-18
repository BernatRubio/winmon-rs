pub mod cpu;
pub mod ram;

use crate::window::Window;

pub trait Metric {
    fn class_name() -> &'static str;
    fn task(hwnd_raw: isize);
}

pub fn spawn_metric<M: Metric>() -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        let mut window = Window::new(M::class_name());
        let class_atom = window.register();
        let hwnd_raw = window.create(class_atom);
        std::thread::spawn(move || M::task(hwnd_raw));
        window.message_loop();
    })
}
