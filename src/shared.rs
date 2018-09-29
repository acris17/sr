use ::std;


extern {
    pub fn system(process: *const std::os::raw::c_char) -> i32;
}


pub fn system_shell(process: &str) {
    if let Ok(process) = std::ffi::CString::new(process) {
        unsafe {
            system(process.as_ptr());
        }
    }
}
pub fn open_url(url: &str, query: &str) {
    let process = format!("open {}{}", url, query);
    system_shell(&process);
}
pub fn quote(string: &str) -> String {
    let string = string.replace("'", "\\'");
    return format!("$'{}'", string);
}
