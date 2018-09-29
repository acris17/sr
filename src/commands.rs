use ::shared;


pub fn search_google(query: &[String]) {
    let query = query
        .join("+")
        .replace("\"", "%22");

    let url = "http://www.google.com/search?q=";
    shared::open_url(url, &query);
}
pub fn search_wikipedia(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://en.wikipedia.org/wiki/Special:Search?search=";
    shared::open_url(url, &query);
}
pub fn search_crates(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://crates.io/search?q=";
    let process = format!("open {}{}", url, query);
    shared::system_shell(&process);
}
pub fn search_rust(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://doc.rust-lang.org/std/index.html?search=";
    shared::open_url(url, &query);
}
pub fn search_youtube(query: &[String]) {
    let query = query
        .join("+");

    let url = "https://www.youtube.com/results?search_query=";
    shared::open_url(url, &query);
}
pub fn search_dictionary(query: &[String]) {
    let query = query
        .join(" ");

    let query = shared::quote(&query);
    let url = "dict://";
    shared::open_url(url, &query);
}


pub fn list_engines() {
    println!("go     = google");
    println!("wiki   = wikipedia");
    println!("crates = rust crates.io");
    println!("rust   = rust documentation");
    println!("yt     = youtube");
    println!("dict   = macOS dictionary");
}
