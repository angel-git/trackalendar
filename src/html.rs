use crate::config::{Config, Theme};
use chrono::{Datelike, Duration, NaiveDate};
use std::collections::HashMap;

#[derive(Clone)]
struct DayCell {
    date: NaiveDate,
    count: u16,
    level: u8,
}

pub fn create_html(entries: &[(NaiveDate, u16)], config: &Config) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">");
    html.push_str(&add_head(config));
    html.push_str("<body>\n");

    let years = extract_years(entries);
    let map: HashMap<NaiveDate, u16> = entries
        .into_iter()
        .map(|(date, count)| (*date, *count))
        .collect();

    html.push_str("<div class=\"container\">\n");
    html.push_str(&format!("<h1>{}</h1>\n", config.title));

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

fn add_head(config: &Config) -> String {
    let css_variables = create_css_variables(&config.theme);
    let styles = r#"

        body {
            font-family: monospace,sans-serif;
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
          grid-template-rows: repeat(7, 24px);
          gap: 2px;
        }
        .day {
          width: 24px;
          height: 24px;
          border-radius: 2px;
        }

        @media (max-width: 1400px) {
            .year {
                grid-template-rows: repeat(7, 18px);
            }
            .day {
                width: 18px;
                height: 18px;
            }
        }

        @media (max-width: 1080px) {
            .year {
                grid-template-rows: repeat(7, 14px);
            }
            .day {
                width: 14px;
                height: 14px;
            }
        }

        .level-0 { background: var(--level-0-color); }
        .level-1 { background: var(--level-1-color); }
        .level-2 { background: var(--level-2-color); }
        .level-3 { background: var(--level-3-color); }
        .level-4 { background: var(--level-4-color); }

        .empty {
          background: transparent;
        }

        "#;
    format!(
        "<head><style>{} {}</style><title>Trackalendar</title></head>\n",
        css_variables,
        styles,
    )
}

fn create_css_variables(theme: &Theme) -> String {
    match theme {
        Theme::Green => r#"
        :root {
        --level-0-color: #ebedf0;
        --level-1-color: #c6e48b;
        --level-2-color: #7bc96f;
        --level-3-color: #239a3b;
        --level-4-color: #196127;
        }
        "#
        .to_string(),
        Theme::GreenReverse => r#"
        :root {
        --level-0-color: #196127;
        --level-1-color: #239a3b;
        --level-2-color: #7bc96f;
        --level-3-color: #c6e48b;
        --level-4-color: #ebedf0;
        }
        "#
        .to_string(),
        Theme::Red => r#"
        :root {
        --level-0-color: #f2e9e9;
        --level-1-color: #f5b5b5;
        --level-2-color: #f26d6d;
        --level-3-color: #d73a3a;
        --level-4-color: #8b1e1e;
        }
        "#
        .to_string(),
        Theme::RedReverse => r#"
        :root {
        --level-0-color: #8b1e1e;
        --level-1-color: #d73a3a;
        --level-2-color: #f26d6d;
        --level-3-color: #f5b5b5;
        --level-4-color: #f2e9e9;
        }
        "#
        .to_string(),
        Theme::Blue => r#"
        :root {
        --level-0-color: #ebf5fb;
        --level-1-color: #b6dcf6;
        --level-2-color: #73bdf0;
        --level-3-color: #2f81f7;
        --level-4-color: #1f4e8c;
        }
        "#
        .to_string(),
        Theme::BlueReverse => r#"
        :root {
        --level-0-color: #1f4e8c;
        --level-1-color: #2f81f7;
        --level-2-color: #73bdf0;
        --level-3-color: #b6dcf6;
        --level-4-color: #ebf5fb;
        }
        "#
        .to_string(),
    }
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
