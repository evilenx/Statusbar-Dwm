use std::fs;
use std::time::Duration;
use chrono::{DateTime, Local};
use sysinfo::{System, SystemExt, ProcessorExt};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(display_name: *const i8) -> *mut std::ffi::c_void;
    fn XStoreName(display: *mut std::ffi::c_void, window: u64, name: *const i8) -> i32;
    fn XDefaultRootWindow(display: *mut std::ffi::c_void) -> u64;
    fn XFlush(display: *mut std::ffi::c_void) -> i32;
}

fn get_battery_status() -> String {
    let output = Command::new("acpi")
        .arg("-b")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            if !output_str.is_empty() {
                let parts: Vec<&str> = output_str.split(',').collect();
                if parts.len() >= 2 {
                    let percent = parts[1].trim();
                    return format!("Bat: {}", percent);
                }
                return format!("Bat: {}", output_str.trim());
            }
            "Bat: ?".to_string()
        },
        _ => "Bat: N/A".to_string()
    }
}

fn get_memory_usage() -> String {
    let output = Command::new("free")
        .arg("-m")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            if lines.len() >= 2 {
                let parts: Vec<&str> = lines[1].split_whitespace().collect();
                if parts.len() >= 3 {
                    let total_mb: u64 = parts[1].parse().unwrap_or(0);
                    let used_mb: u64 = parts[2].parse().unwrap_or(0);

                    if total_mb > 0 {
                        let total_gb = total_mb as f64 / 1024.0;
                        let used_gb = used_mb as f64 / 1024.0;
                        let percent = (used_mb as f64 / total_mb as f64) * 100.0;

                        return format!("RAM: {:.1}GB/{:.1}GB ({:.0}%)", used_gb, total_gb, percent);
                    }
                }
            }
            "RAM: N/A".to_string()
        },
        _ => "RAM: N/A".to_string()
    }
}

fn get_wifi_ssid() -> String {
    let output = Command::new("nmcli")
        .args(&["-t", "-f", "active,ssid", "dev", "wifi"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            for line in output_str.lines() {
                if line.starts_with("yes:") {
                    let ssid = line.trim_start_matches("yes:");
                    if !ssid.is_empty() {
                        return format!("WiFi: {}", ssid);
                    }
                }
            }
            "WiFi: Disconnected".to_string()
        },
        _ => "WiFi: N/A".to_string()
    }
}

fn get_keyboard_layout() -> String {
    let output = Command::new("setxkbmap")
        .arg("-query")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            if let Some(layout_line) = output_str.lines().find(|line| line.contains("layout")) {
                let parts: Vec<&str> = layout_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return format!("{}", parts[1].to_uppercase());
                }
            }
            "?".to_string()
        },
        _ => "N/A".to_string()
    }
}

fn read_network_stats(interface: &str) -> Result<(u64, u64), std::io::Error> {
    let file = fs::File::open("/proc/net/dev")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(interface) {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let stats: Vec<&str> = parts[1].split_whitespace().collect();
                if stats.len() >= 9 {
                    let rx_bytes = stats[0].parse::<u64>().unwrap_or(0);
                    let tx_bytes = stats[8].parse::<u64>().unwrap_or(0);
                    return Ok((rx_bytes, tx_bytes));
                }
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Interface not found"))
}

fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec >= 1_048_576.0 {
        format!("{:.1}MB/s", bytes_per_sec / 1_048_576.0)
    } else if bytes_per_sec >= 1024.0 {
        format!("{:.0}KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.0}B/s", bytes_per_sec)
    }
}

fn get_active_interface() -> Option<String> {
    let interfaces = vec!["wlan0", "wlp3s0", "wlp2s0", "eth0", "enp0s3", "enp0s25"];
    
    for interface in interfaces {
        if let Ok((rx, tx)) = read_network_stats(interface) {
            if rx > 0 || tx > 0 {
                return Some(interface.to_string());
            }
        }
    }
    None
}

fn network_speed_thread(speed_data: Arc<Mutex<(String, String)>>) {
    let mut last_rx: u64 = 0;
    let mut last_tx: u64 = 0;
    let mut last_time = std::time::Instant::now();

    loop {
        if let Some(interface) = get_active_interface() {
            if let Ok((rx, tx)) = read_network_stats(&interface) {
                let now = std::time::Instant::now();
                let elapsed = now.duration_since(last_time).as_secs_f64();

                if elapsed > 0.0 && last_rx > 0 && last_tx > 0 {
                    let rx_speed = ((rx - last_rx) as f64) / elapsed;
                    let tx_speed = ((tx - last_tx) as f64) / elapsed;

                    let mut speed = speed_data.lock().unwrap();
                    *speed = (
                        format!("↓{}", format_speed(rx_speed)),
                        format!("↑{}", format_speed(tx_speed))
                    );
                }

                last_rx = rx;
                last_tx = tx;
                last_time = now;
            }
        } else {
            let mut speed = speed_data.lock().unwrap();
            *speed = ("↓0B/s".to_string(), "↑0B/s".to_string());
        }

        thread::sleep(Duration::from_nanos((1_000_000_000u64 / 144) as u64));
    }
}

fn cpu_monitoring_thread(cpu_data: Arc<Mutex<String>>) {
    let mut system = System::new_all();

    loop {
        system.refresh_cpu();
        thread::sleep(Duration::from_millis(100));
        system.refresh_cpu();
        
        let usage = format!("CPU: {:.0}%", system.global_processor_info().cpu_usage());

        let mut cpu = cpu_data.lock().unwrap();
        *cpu = usage;

        thread::sleep(Duration::from_nanos((1_000_000_000u64 / 144) as u64));
    }
}

fn keyboard_layout_thread(layout_data: Arc<Mutex<String>>) {
    loop {
        let layout = get_keyboard_layout();
        let mut layout_lock = layout_data.lock().unwrap();
        *layout_lock = layout;

        thread::sleep(Duration::from_nanos((1_000_000_000u64 / 144) as u64));
    }
}

fn wifi_monitor_thread(wifi_data: Arc<Mutex<String>>) {
    loop {
        let ssid = get_wifi_ssid();
        let mut wifi = wifi_data.lock().unwrap();
        *wifi = ssid;

        thread::sleep(Duration::from_nanos((1_000_000_000u64 / 144) as u64));
    }
}

fn system_stats_thread(memory_data: Arc<Mutex<String>>, battery_data: Arc<Mutex<String>>) {
    loop {
        let memory = get_memory_usage();
        let mut mem_lock = memory_data.lock().unwrap();
        *mem_lock = memory;
        drop(mem_lock);

        let battery = get_battery_status();
        let mut bat_lock = battery_data.lock().unwrap();
        *bat_lock = battery;

        thread::sleep(Duration::from_nanos((1_000_000_000u64 / 144) as u64));
    }
}

fn main() {
    let disp = unsafe { XOpenDisplay(std::ptr::null()) };
    if disp.is_null() {
        eprintln!("Error: No se puede abrir X11 display");
        return;
    }

    let root = unsafe { XDefaultRootWindow(disp) };

    let speed_data = Arc::new(Mutex::new(("↓0B/s".to_string(), "↑0B/s".to_string())));
    let cpu_data = Arc::new(Mutex::new("CPU: 0%".to_string()));
    let wifi_data = Arc::new(Mutex::new("WiFi: --".to_string()));
    let memory_data = Arc::new(Mutex::new("RAM: --".to_string()));
    let battery_data = Arc::new(Mutex::new("Bat: --".to_string()));
    let layout_data = Arc::new(Mutex::new("US".to_string()));

    let speed_data_clone = speed_data.clone();
    thread::spawn(move || {
        network_speed_thread(speed_data_clone);
    });

    let cpu_data_clone = cpu_data.clone();
    thread::spawn(move || {
        cpu_monitoring_thread(cpu_data_clone);
    });

    let wifi_data_clone = wifi_data.clone();
    thread::spawn(move || {
        wifi_monitor_thread(wifi_data_clone);
    });

    let memory_data_clone = memory_data.clone();
    let battery_data_clone = battery_data.clone();
    thread::spawn(move || {
        system_stats_thread(memory_data_clone, battery_data_clone);
    });

    let layout_data_clone = layout_data.clone();
    thread::spawn(move || {
        keyboard_layout_thread(layout_data_clone);
    });

    loop {
        let local: DateTime<Local> = Local::now();
        let time_string = local.format("%d-%m-%Y %T.%3f").to_string();

        let speed = speed_data.lock().unwrap().clone();
        let cpu_usage = cpu_data.lock().unwrap().clone();
        let wifi_ssid = wifi_data.lock().unwrap().clone();
        let memory_usage = memory_data.lock().unwrap().clone();
        let battery_status = battery_data.lock().unwrap().clone();
        let kb_layout = layout_data.lock().unwrap().clone();

        let status_string = format!("[{}] {} | {} | {} {} | {} | {} | {}",
                                    kb_layout, time_string, wifi_ssid, 
                                    speed.0, speed.1, battery_status, 
                                    memory_usage, cpu_usage);

        let c_string = format!("{}\0", status_string);

        unsafe {
            XStoreName(disp, root, c_string.as_ptr() as *const i8);
            XFlush(disp);
        }

        std::thread::sleep(Duration::from_nanos((1_000_000_000u64 / 144) as u64));
    }
} 
