use ::shared;


pub fn google(query: &[String]) {
    let query = query
        .join("+")
        .replace("\"", "%22");

    let url = "http://www.google.com/search?q=";
    shared::open_url(url, &query);
}
pub fn wikipedia(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://en.wikipedia.org/wiki/Special:Search?search=";
    shared::open_url(url, &query);
}
pub fn crates_io(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://crates.io/search?q=";
    let process = format!("open {}{}", url, query);
    shared::system_shell(&process);
}
pub fn rust_docs(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://doc.rust-lang.org/std/index.html?search=";
    shared::open_url(url, &query);
}
pub fn youtube(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://www.youtube.com/results?search_query=";
    shared::open_url(url, &query);
}
pub fn dict(query: &[String]) {
    let query = query
        .join(" ");

    let query = shared::quote(&query);
    let url = "dict://";
    shared::open_url(url, &query);
}
