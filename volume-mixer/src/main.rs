use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Button, Label,
    Orientation, Scale, Separator,
};
use pulsectl::controllers::{AppControl, DeviceControl, SinkController};
#[allow(unused_imports)]
use pulsectl::controllers::types::*;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "com.dwm.volume-mixer";

const CSS: &str = "
window {
    background-color: #0a0a0a;
    border: 1px solid #6d28d9;
    border-radius: 8px;
}
.row {
    padding: 3px 10px;
}
.app-label {
    font-family: 'Iosevka', 'JetBrains Mono', monospace;
    font-size: 11px;
    color: #777;
    min-width: 100px;
}
.vol-pct {
    font-family: 'Iosevka', 'JetBrains Mono', monospace;
    font-size: 11px;
    color: #444;
    min-width: 34px;
}
.mute-btn {
    background: none;
    border: none;
    font-size: 12px;
    padding: 0 2px;
    min-width: 22px;
    min-height: 22px;
    color: #444;
}
.mute-btn.muted { color: #7c3aed; }
.mute-btn:hover { color: #8b5cf6; }
.section {
    font-family: 'Iosevka', 'JetBrains Mono', monospace;
    font-size: 9px;
    letter-spacing: 2px;
    color: #2a2a2a;
    padding: 5px 10px 1px 10px;
}
scale trough {
    background-color: #181818;
    min-height: 3px;
    border-radius: 2px;
}
scale trough highlight {
    background-color: #6d28d9;
    border-radius: 2px;
}
scale slider {
    background-color: #8b5cf6;
    min-width: 10px;
    min-height: 10px;
    border-radius: 50%;
    margin: -4px 0;
}
separator {
    background-color: #161616;
    min-height: 1px;
    margin: 1px 0;
}
";

fn vol_icon(v: f64, muted: bool) -> &'static str {
    if muted || v == 0.0 { "M" } else if v < 34.0 { "-" } else if v < 67.0 { "~" } else { "+" }
}

fn pa_pct(v: u32) -> f64 {
    (v as f64 / 65536.0 * 100.0).clamp(0.0, 150.0)
}

fn make_row(
    name: &str, vol: f64, muted: bool,
    on_vol: impl Fn(f64) + 'static,
    on_mute: impl Fn() + 'static,
) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, 4);
    row.add_css_class("row");

    let lbl = Label::new(Some(name));
    lbl.add_css_class("app-label");
    lbl.set_xalign(0.0);
    lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);
    lbl.set_max_width_chars(14);
    row.append(&lbl);

    let scale = Scale::with_range(Orientation::Horizontal, 0.0, 150.0, 1.0);
    scale.set_value(vol);
    scale.set_hexpand(true);
    scale.set_draw_value(false);
    scale.add_mark(100.0, gtk::PositionType::Bottom, None);
    row.append(&scale);

    let pct = Label::new(Some(&format!("{:.0}%", vol)));
    pct.add_css_class("vol-pct");
    row.append(&pct);

    let btn = Button::with_label(vol_icon(vol, muted));
    btn.add_css_class("mute-btn");
    if muted { btn.add_css_class("muted"); }
    row.append(&btn);

    let pct2 = pct.clone();
    let btn2 = btn.clone();
    let mc   = Rc::new(RefCell::new(muted));
    let mc2  = mc.clone();

    scale.connect_value_changed(move |s| {
        let v = s.value();
        pct2.set_text(&format!("{:.0}%", v));
        btn2.set_label(vol_icon(v, *mc2.borrow()));
        on_vol(v);
    });

    let s2 = scale.clone();
    let p2 = pct.clone();
    btn.connect_clicked(move |b| {
        on_mute();
        let mut m = mc.borrow_mut();
        *m = !*m;
        let v = s2.value();
        p2.set_text(&format!("{:.0}%", v));
        b.set_label(vol_icon(v, *m));
        if *m { b.add_css_class("muted"); } else { b.remove_css_class("muted"); }
    });

    row
}

fn build_content() -> GtkBox {
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    let s1 = Label::new(Some("OUTPUT"));
    s1.add_css_class("section");
    s1.set_xalign(0.0);
    vbox.append(&s1);

    if let Ok(mut c) = SinkController::create() {
        if let Ok(sinks) = c.list_devices() {
            for sink in sinks {
                let idx   = sink.index;
                let name  = sink.description.clone()
                    .unwrap_or_else(|| sink.name.clone().unwrap_or("Output".into()));
                let vol   = pa_pct(sink.volume.get()[0].0);
                let muted = sink.mute;
                vbox.append(&make_row(&name, vol, muted,
                    move |p| { std::process::Command::new("pactl")
                        .args(["set-sink-volume", &idx.to_string(), &format!("{:.0}%", p)])
                        .output().ok(); },
                    move || { std::process::Command::new("pactl")
                        .args(["set-sink-mute", &idx.to_string(), "toggle"])
                        .output().ok(); },
                ));
            }
        }
    }

    vbox.append(&Separator::new(Orientation::Horizontal));

    let s2 = Label::new(Some("APPS"));
    s2.add_css_class("section");
    s2.set_xalign(0.0);
    vbox.append(&s2);

    if let Ok(mut c) = SinkController::create() {
        match c.list_applications() {
            Ok(apps) if !apps.is_empty() => {
                for app in apps {
                    let idx   = app.index;
                    let name  = app.proplist.get_str("application.name")
                        .unwrap_or_else(|| "App".into());
                    let vol   = pa_pct(app.volume.get()[0].0);
                    let muted = app.mute;
                    vbox.append(&make_row(&name, vol, muted,
                        move |p| { std::process::Command::new("pactl")
                            .args(["set-sink-input-volume", &idx.to_string(), &format!("{:.0}%", p)])
                            .output().ok(); },
                        move || { std::process::Command::new("pactl")
                            .args(["set-sink-input-mute", &idx.to_string(), "toggle"])
                            .output().ok(); },
                    ));
                }
            }
            _ => {
                let l = Label::new(Some("no streams"));
                l.add_css_class("section");
                vbox.append(&l);
            }
        }
    }

    vbox
}

fn build_ui(app: &Application) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS);
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let win = ApplicationWindow::builder()
        .application(app)
        .title("volume-mixer")
        .default_width(320)
        .default_height(1)
        .resizable(false)
        .decorated(false)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 0);
    root.set_margin_top(6);
    root.set_margin_bottom(6);

    root.append(&build_content());

    win.set_child(Some(&root));
    win.present();

    // ESC cierra
    let evk  = gtk::EventControllerKey::new();
    let win2 = win.clone();
    evk.connect_key_pressed(move |_, key, _, _| {
        if key == gtk::gdk::Key::Escape {
            win2.close();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    win.add_controller(evk);

}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}
