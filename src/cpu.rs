use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt, ProcessorExt};


pub struct CpuData {
    pub usage: String
}

pub fn monitor(data: Arc<Mutex<CpuData>>) {
    let mut sys = System::new_all();
    loop {
        sys.refresh_cpu();
        thread::sleep(Duration::from_millis(100));
        sys.refresh_cpu();
        
        let usage = format!("CPU: {:.04}%", sys.global_processor_info().cpu_usage());
        *data.lock().unwrap() = CpuData{usage};
				std::thread::sleep(Duration::from_nanos((1e9 / 144.) as u64));
    }
}
