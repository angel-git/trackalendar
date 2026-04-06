use crate::config::parse_config;
use crate::entry::parse_entries_from_file;
use crate::html::create_html;
use std::env;

mod config;
mod entry;
mod html;

fn main() {
    let args: Vec<String> = env::args().collect();
    let events_path = args[1].as_str();
    let config = parse_config();
    let entries = parse_entries_from_file(events_path);
    let html = create_html(&entries, config.title.as_str());
    std::fs::write("output/index.html", html).expect("Failed to write HTML file");
}
