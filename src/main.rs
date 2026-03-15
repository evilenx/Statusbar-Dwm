mod cpu;
mod stats;
mod volume;
mod x11;

use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Local;
use cpu::CpuData;
use stats::StatsData;
use volume::VolumeData;
use x11::X11;
use std::time::Duration;

fn main() {
    let x11 = X11::new().expect("X11 failed");

    let cpu = Arc::new(Mutex::new(CpuData {
        usage: "CPU: --%".into(),
    }));
    let stats = Arc::new(Mutex::new(StatsData {
        battery: "Bat: --".into(),
        memory:  "RAM: --".into(),
    }));
    let vol = Arc::new(Mutex::new(VolumeData {
        display: "🔊 --%".into(),
    }));

    thread::spawn({ let c = cpu.clone();   move || cpu::monitor(c)    });
    thread::spawn({ let s = stats.clone(); move || stats::monitor(s)  });
    thread::spawn({ let v = vol.clone();   move || volume::monitor(v) });

    loop {
        let time  = Local::now().format("%d-%m-%Y %T.%3f").to_string();
        let stats = stats.lock().unwrap();
        let vol   = vol.lock().unwrap();
        let cpu   = cpu.lock().unwrap();

        let bar = format!(
            "{time} | {} | {} | {} | {}",
            stats.battery,
            stats.memory,
            cpu.usage,
            vol.display,
        );

        x11.set_title(&format!("{bar}\0"));
        drop(stats);
        drop(vol);
        drop(cpu);

        std::thread::sleep(Duration::from_nanos((1e9 / 144.) as u64));
    }
}
