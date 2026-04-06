use crate::entry::parse_entries_from_file;
use crate::html::create_html;
use std::env;

mod entry;
mod html;

fn main() {
    let args: Vec<String> = env::args().collect();
    let events_path = args[1].as_str();
    let title = "My count of something"; // TODO get this from config file
    let entries = parse_entries_from_file(events_path);
    let html = create_html(&entries, title);
    std::fs::write("output/index.html", html).expect("Failed to write HTML file");
}
