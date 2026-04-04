use crate::entry::parse_entries_from_file;
use crate::html::create_html;

mod entry;
mod html;

fn main() {
    let title = "My count of something"; // TODO get this from config file
    let entries = parse_entries_from_file("example-events/events.txt"); // TODO get this from config file
    let html = create_html(&entries, title);
    std::fs::write("output/index.html", html).expect("Failed to write HTML file");
}
