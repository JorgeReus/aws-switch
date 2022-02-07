use chrono::{offset::Local, offset::TimeZone};
use colored::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use skim::prelude;
use std::{collections::BTreeMap, ops::Sub};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

mod util;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?:token|secret)").unwrap();
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub attributes: BTreeMap<String, String>,
}

impl prelude::SkimItem for Item {
    fn text(&self) -> prelude::Cow<str> {
        prelude::Cow::Borrowed(&self.name)
    }

    fn preview(&self, _context: prelude::PreviewContext) -> prelude::ItemPreview {
        let mut s: String = "".to_owned();
        for (k, v) in self.attributes.iter() {
            match generate_attributes_entry(&k, &v) {
                Some(entry) => s.push_str(entry.as_str()),
                _ => {}
            };
        }
        prelude::ItemPreview::AnsiText(s)
    }
}

fn generate_attributes_entry(key: &str, v: &str) -> Option<String> {
    let k = &util::capitalize(key.split("_").join(" ").as_str());
    if RE.is_match(key) {
        return None;
    }

    match key {
        "expires_time" => {
            let timestamp = v.parse::<i64>();
            if timestamp.is_err() {
                return Some(
                    format!("Cannot parse field {}, it isn't an unix timestamp", k).to_string(),
                );
            }
            let now = chrono::offset::Local::now();
            let date_time = Local.timestamp(timestamp.unwrap(), 0);
            let mut output_text = format!("{} : {}{}", k, date_time, LINE_ENDING);
            output_text.push_str("Status : ");
            if date_time.lt(&now) {
                output_text.push_str(format!("{}{}", "Expired".red(), LINE_ENDING).as_str());
            } else {
                let minutes_to_expire = date_time.sub(now).num_minutes();
                if minutes_to_expire > 60 {
                    output_text.push_str(
                        format!(
                            "Will expire in {} hours{}",
                            minutes_to_expire / 60,
                            LINE_ENDING
                        )
                        .green()
                        .to_string()
                        .as_str(),
                    )
                } else {
                    output_text.push_str(
                        format!(
                            "Will expire in {} minutes{}",
                            minutes_to_expire, LINE_ENDING
                        )
                        .green()
                        .to_string()
                        .as_str(),
                    )
                }
            }
            return Some(output_text);
        }
        "issued_time" => {
            let timestamp = Local.timestamp(v.parse::<i64>().unwrap(), 0);
            return Some(format!("{} : {}{}", k, timestamp, LINE_ENDING));
        }
        _ => Some(format!("{} : {}{}", k, v, LINE_ENDING)),
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use chrono::Duration;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_generate_attributes_entry() {
        assert_eq!(generate_attributes_entry("aws_secret_access_key", "XXXXXXXXXXX"), None);
        assert_eq!(generate_attributes_entry("aws_session_token", "XXXXXXXXXXX"), None);

        assert_eq!(generate_attributes_entry("aws_access_key_id", "XXXXXXXXXXX"), Some(format!("AWS access key id : XXXXXXXXXXX{}", LINE_ENDING).to_string()));
    }

    #[test]
    fn test_valid_issued_and_expires_time() {
        assert_eq!(generate_attributes_entry("issued_time", "1644192523"), Some(format!("Issued time : 2022-02-06 18:08:43 -06:00{}", LINE_ENDING).to_string()));

        // Test expired time
        let expires_time_expired = generate_attributes_entry("expires_time", "1644192523").unwrap();
        let parts: Vec<&str> = expires_time_expired.split(LINE_ENDING).collect();
        assert_eq!(parts[0], "Expires time : 2022-02-06 18:08:43 -06:00");
        assert_eq!(parts[1].contains("Status : "), true);
        assert_eq!(parts[1].contains("Expired"), true);

        // Test valid time
        let now = chrono::offset::Local::now().add(Duration::minutes(3)).timestamp();
        let expires_time_expired = generate_attributes_entry("expires_time", now.to_string().as_str()).unwrap();
        let parts: Vec<&str> = expires_time_expired.split(LINE_ENDING).collect();
        assert_eq!(parts[0].contains("Expires time :"), true);
        assert_eq!(parts[1].contains("Status : "), true);
        assert_eq!(parts[1].contains("Will expire in "), true);
    }
}
