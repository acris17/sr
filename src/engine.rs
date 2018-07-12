use ::std;


pub fn google(query: &[String]) {
    let query = query
        .join("+")
        .replace("\"", "%22");

    let url = "http://www.google.com/search?q=";
    open_url(url, &query);
}
pub fn wikipedia(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://en.wikipedia.org/wiki/Special:Search?search=";
    open_url(url, &query);
}
pub fn crates_io(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://crates.io/search?q=";
    let process = format!("open {}{}", url, query);
    system_shell(&process);
}
pub fn rust_docs(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://doc.rust-lang.org/std/index.html?search=";
    open_url(url, &query);
}
pub fn youtube(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://www.youtube.com/results?search_query=";
    open_url(url, &query);
}
pub fn dict(query: &[String]) {
    let query = query
        .join(" ");

    let query = quote(&query);
    let url = "dict://";
    open_url(url, &query);
}


extern {
    pub fn system(process: *const std::os::raw::c_char) -> i32;
}
fn system_shell(process: &str) {
    if let Ok(process) = std::ffi::CString::new(process) {
        unsafe {
            system(process.as_ptr());
        }
    }
}
fn open_url(url: &str, query: &str) {
    let process = format!("open {}{}", url, query);
    system_shell(&process);
}
fn quote(string: &str) -> String {
    let string = string.replace("'", "\\'");
    return format!("$'{}'", string);
}
