use crate::metrics::cpu::Cpu;
use crate::metrics::ram::Ram;
use crate::metrics::spawn_metric;

pub fn run() {
    let handles = vec![spawn_metric::<Cpu>(), spawn_metric::<Ram>()];

    for handle in handles {
        handle.join().unwrap();
    }
}
