use std::ffi::CString;

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(name: *const i8) -> *mut std::ffi::c_void;
    fn XStoreName(d: *mut std::ffi::c_void, w: u64, name: *const i8) -> i32;
    fn XDefaultRootWindow(d: *mut std::ffi::c_void) -> u64;
    fn XFlush(d: *mut std::ffi::c_void) -> i32;
}

pub struct X11 {
    display: *mut std::ffi::c_void,
    root: u64
}

impl X11 {
    pub fn new() -> Result<Self, &'static str> {
        unsafe {
            let d = XOpenDisplay(std::ptr::null());
            if d.is_null() { return Err("X11 failed"); }
            Ok(X11 { display: d, root: XDefaultRootWindow(d) })
        }
    }

		pub fn set_title(&self, title: &str) {
				// Remove null bytes and control characters before CString conversion
				let clean_title: String = title
						.bytes()
						.filter(|&b| b != 0)  // Remove null bytes
						.filter(|&b| b >= 32 || b == b'\n' || b == b'\t')  // Keep printable chars
						.map(|b| b as char)
						.collect();

				// Fallback to safe default if empty
				let safe_title = if clean_title.is_empty() {
						"Statusbar"
				} else {
						&clean_title
				};

				match CString::new(safe_title) {
						Ok(cstr) => unsafe {
								XStoreName(self.display, self.root, cstr.as_ptr());
								XFlush(self.display);
						},
						Err(_) => unsafe {
								// Emergency fallback
								let fallback = CString::new("Statusbar").unwrap();
								XStoreName(self.display, self.root, fallback.as_ptr());
								XFlush(self.display);
						}
				}
		}
}
