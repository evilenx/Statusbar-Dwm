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

												return format!("RAM: {:.04}GB/{:.04}GB ({:.0}%)", used_gb, total_gb, percent);
										}
								}
						}
						"RAM: N/A".to_string()
				},
				_ => "RAM: N/A".to_string()
		}
}
