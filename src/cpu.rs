use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, ProcessorExt};
 
pub struct CpuData {
    pub usage: String,
}
 
pub fn monitor(data: Arc<Mutex<CpuData>>) {
    let mut sys = System::new_all();
    let target_period = Duration::from_millis(500);
 
    // Primer refresh para inicializar contadores
    sys.refresh_cpu();
    thread::sleep(Duration::from_millis(100));
 
    loop {
        let now = Instant::now();
 
        sys.refresh_cpu();
        let usage = format!("CPU: {:.1}%", sys.global_processor_info().cpu_usage());
        *data.lock().unwrap() = CpuData { usage };
 
        let elapsed = now.elapsed();
        if elapsed < target_period {
            thread::sleep(target_period - elapsed);
        }
    }
}
 
