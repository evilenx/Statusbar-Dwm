use std::sync::{Arc, Mutex};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct VolumeData {
    pub display: String,
}

pub fn monitor(data: Arc<Mutex<VolumeData>>) {
    loop {
        *data.lock().unwrap() = VolumeData {
            display: get_display(),
        };
        thread::sleep(Duration::from_millis(500));
    }
}

fn get_display() -> String {
    let muted = is_muted();
    let vol   = get_percent();

    let icon = if muted || vol == 0 { "🔇" }
               else if vol < 34     { "🔈" }
               else if vol < 67     { "🔉" }
               else                 { "🔊" };

    if muted {
        format!("{} MUTE", icon)
    } else {
        format!("{} {}%", icon, vol)
    }
}

fn get_percent() -> u32 {
    Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            String::from_utf8(o.stdout).ok()?
                .split('/')
                .nth(1)?
                .trim()
                .trim_end_matches('%')
                .parse().ok()
        })
        .unwrap_or(0)
}

fn is_muted() -> bool {
    Command::new("pactl")
        .args(["get-sink-mute", "@DEFAULT_SINK@"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("yes"))
        .unwrap_or(false)
}
