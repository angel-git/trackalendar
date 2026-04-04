use chrono::NaiveDate;
use std::collections::HashMap;

pub fn parse_entries_from_file(filename: &str) -> Vec<(NaiveDate, u16)> {
    let content = read_entry_file(filename);
    parse_entries(&content)
}

fn read_entry_file(filename: &str) -> String {
    std::fs::read_to_string(filename).expect(&format!("Failed to read file: {}", filename))
}

fn parse_entries(entries: &str) -> Vec<(NaiveDate, u16)> {
    let mut counts: HashMap<NaiveDate, u16> = HashMap::new();

    for line in entries.lines().filter(|l| !l.trim().is_empty()) {
        let date = NaiveDate::parse_from_str(line, "%Y-%m-%d")
            .expect(&format!("Invalid date format: {}", line));

        *counts.entry(date).or_insert(0) += 1;
    }

    // Convert to Vec and sort DESC by count
    let mut result: Vec<(NaiveDate, u16)> = counts.into_iter().collect();

    result.sort_by(|a, b| b.0.cmp(&a.0));
    result
}

#[cfg(test)]
mod tests {
    use crate::entry::parse_entries;

    #[test]
    fn should_return_dates_sorted_and_grouped() {
        let entries = r#"
        2024-01-03
        2024-01-01
        2024-01-01
        2024-01-02
        "#;
        let parsed = parse_entries(entries);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed.get(0).unwrap().0.to_string().as_str(), "2024-01-03");
        assert_eq!(parsed.get(0).unwrap().1, 1);
        assert_eq!(parsed.get(1).unwrap().0.to_string().as_str(), "2024-01-02");
        assert_eq!(parsed.get(1).unwrap().1, 1);
        assert_eq!(parsed.get(2).unwrap().0.to_string().as_str(), "2024-01-01");
        assert_eq!(parsed.get(2).unwrap().1, 2);
    }

    #[test]
    fn should_skip_empty_lines() {
        let entries = r#"

        2024-01-01

        2024-01-02


        "#;
        let parsed = parse_entries(entries);
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    #[should_panic(expected = "Invalid date format")]
    fn should_return_error_on_wrong_entries() {
        let entries = r#"
        2024-01-999
        "#;
        parse_entries(entries);
    }
}
