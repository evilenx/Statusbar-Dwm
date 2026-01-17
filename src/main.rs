mod cpu;
mod stats;
mod x11;

use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Local;
use cpu::CpuData;
use stats::StatsData;
use x11::X11;

use std::time::Duration;
fn main() {
    let x11 = X11::new().expect("X11 failed");
    
    let cpu = Arc::new(Mutex::new(CpuData{usage: "CPU: 0%".into()}));
    let stats = Arc::new(Mutex::new(StatsData{
        battery: "Bat: --".into(),
        memory: "RAM: --".into()
    }));

    thread::spawn({let c=cpu.clone(); move|| cpu::monitor(c)});
    thread::spawn({let s=stats.clone(); move|| stats::monitor(s)});

    loop {
        let time = Local::now().format("%d-%m-%Y %T.%3f").to_string();
        let stats = stats.lock().unwrap();
        
        let bar = format!("{time} | {} | {} | {}",
                         stats.battery, stats.memory, cpu.lock().unwrap().usage);
        
        x11.set_title(&format!("{bar}\0"));
				std::thread::sleep(Duration::from_nanos((1e9 / 144.) as u64));
	
    }
}
