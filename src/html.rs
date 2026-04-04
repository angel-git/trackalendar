use chrono::NaiveDate;

pub fn create_html(entries: &[(NaiveDate, u16)], title: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">");
    html.push_str(&add_head());
    html.push_str("<body>\n");
    html.push_str(&format!("<h1>{}</h1>\n", title));
    html.push_str("<table border=\"1\"><tr><th>Date</th><th>Count</th></tr>\n");

    for (date, count) in entries {
        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>\n", date, count));
    }

    html.push_str("</table></body></html>");
    html
}

fn add_head() -> String {
    let styles = r#"

        body {
            background-color: oklch(0.9674 0 214.73);
        }

        "#;
    format!(
        "<head><style>{}</style><title>Trackalendar</title></head>\n",
        styles,
    )
}
