use chrono::{Datelike, Duration, NaiveDate};
use std::collections::HashMap;

#[derive(Clone)]
struct DayCell {
    date: NaiveDate,
    count: u16,
    level: u8,
}

pub fn create_html(entries: &[(NaiveDate, u16)], title: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">");
    html.push_str(&add_head());
    html.push_str("<body>\n");

    let years = extract_years(entries);
    let map: HashMap<NaiveDate, u16> = entries
        .into_iter()
        .map(|(date, count)| (*date, *count))
        .collect();


    html.push_str("<div class=\"container\">\n");
    html.push_str(&format!("<h1>{}</h1>\n", title));

    for year in years {
        html.push_str(&format!("<h3>{}</h3>\n", year));
        // html.push_str("<div>\n");

        let weeks = build_year(year, &map);
        html.push_str("<div class=\"year\">\n");

        for week in weeks {
            for day in week {
                if let Some(d) = day {
                    html.push_str(&format!(
                        "<div class=\"day level-{}\" title=\"{} ({} events)\"></div>\n",
                        d.level, d.date, d.count
                    ));
                } else {
                    html.push_str("<div class=\"day empty\"></div>\n");
                }
            }
        }
        html.push_str("</div>\n");
    }
    html.push_str("</div>\n");
    html.push_str("</body></html>");
    html
}

fn add_head() -> String {
    let styles = r#"

        body {
            background-color: oklch(0.9674 0 214.73);
        }
        h3 {
            margin-bottom: 0;
        }
        .container {
          display: flex;
          flex-direction: column;
          align-items: center;
          gap: 4px;
        }

        .year {
          display: grid;
          grid-auto-flow: column;
          grid-template-rows: repeat(7, 12px);
          gap: 2px;
        }

        .day {
          width: 12px;
          height: 12px;
          border-radius: 2px;
        }

        /* GitHub-like colors */
        .level-0 { background: #ebedf0; }
        .level-1 { background: #c6e48b; }
        .level-2 { background: #7bc96f; }
        .level-3 { background: #239a3b; }
        .level-4 { background: #196127; }

        .empty {
          background: transparent;
        }

        "#;
    format!(
        "<head><style>{}</style><title>Trackalendar</title></head>\n",
        styles,
    )
}

fn extract_years(entries: &[(NaiveDate, u16)]) -> Vec<i32> {
    let mut years: Vec<i32> = entries.iter().map(|(date, _)| date.year()).collect();
    years.sort_unstable();
    years.dedup();
    years
}

fn build_year(year: i32, data: &HashMap<NaiveDate, u16>) -> Vec<Vec<Option<DayCell>>> {
    let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

    let mut weeks: Vec<Vec<Option<DayCell>>> = Vec::new();
    let mut current_week: Vec<Option<DayCell>> = vec![None; 7];

    let mut current = start;

    // Offset first week
    let offset = start.weekday().num_days_from_monday() as usize;

    for i in 0..offset {
        current_week[i] = None;
    }

    while current <= end {
        let weekday = current.weekday().num_days_from_monday() as usize;

        let count = *data.get(&current).unwrap_or(&0);

        let level = match count {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            _ => 4,
        };

        current_week[weekday] = Some(DayCell {
            date: current,
            count,
            level,
        });

        if weekday == 6 {
            weeks.push(current_week);
            current_week = vec![None; 7];
        }

        current += Duration::days(1);
    }

    // Push last week if needed
    if current_week.iter().any(|d| d.is_some()) {
        weeks.push(current_week);
    }

    weeks
}
