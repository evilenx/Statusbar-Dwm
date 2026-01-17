use std::sync::{Arc, Mutex};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct StatsData {
    pub battery: String,
    pub memory: String
}

pub fn monitor(data: Arc<Mutex<StatsData>>) {
    loop {
        *data.lock().unwrap() = StatsData {
            battery: get_battery(),
            memory: get_memory()
        };
        thread::sleep(Duration::from_nanos(1_000_000_000u64/144));
    }
}

fn get_battery() -> String {
    match Command::new("acpi").arg("-b").output() {
        Ok(o) if o.status.success() => {
            let output = String::from_utf8_lossy(&o.stdout);
            let parts: Vec<&str> = output.split(',').map(|s| s.trim()).collect();
            if let Some(p) = parts.get(1) {
                format!("Bat: {}", p)
            } else {
                format!("Bat: {}", output.trim())
            }
        },
        _ => "Bat: N/A".to_string()
    }
}

fn get_memory() -> String {
    match Command::new("free").arg("-m").output() {
        Ok(o) if o.status.success() => {
            let output = String::from_utf8_lossy(&o.stdout);
            let lines: Vec<&str> = output.lines().collect();
            if lines.len() < 2 { return "RAM: N/A".to_string(); }
            
            let parts: Vec<&str> = lines[1].split_whitespace().collect();
            if parts.len() < 3 { return "RAM: N/A".to_string(); }
            
            let total = match parts[1].parse::<u64>() {
                Ok(n) => n, _ => return "RAM: N/A".to_string()
            };
            let used = match parts[2].parse::<u64>() {
                Ok(n) => n, _ => return "RAM: N/A".to_string()
            };
            
            if total == 0 { return "RAM: N/A".to_string(); }
            
            let t_gb = total as f64/1024.0;
            let u_gb = used as f64/1024.0;
            format!("RAM: {:.1}G/{:.1}G ({:.0}%)", u_gb, t_gb, u_gb/total as f64*100.0)
        },
        _ => "RAM: N/A".to_string()
    }
}
