#![allow(dead_code)]

use chrono::{DateTime, FixedOffset};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::{collections::HashMap, collections::HashSet, net::IpAddr, path::Path};
use crate::error::{Result,map_error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub ip: IpAddr,
    #[serde(with = "date_serialization")]
    pub timestamp: DateTime<FixedOffset>,
    pub path: String,
    pub method: String,
    pub status_code: u16,
    pub referrer: String,
    pub user_agent: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestStats {
    pub by_day: Vec<(String, i32)>,
    pub unique_by_day: Vec<(String, i32)>,
    pub by_path: Vec<(String, i32)>,
    pub requests: Vec<Request>,
}

impl RequestStats {
    pub fn from_requests(requests: Vec<Request>) -> Self {
        let mut stats = RequestStats {
            requests,
            ..RequestStats::default()
        };
        stats.update();
        stats
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(map_error)?;
        Ok(from_str(&content).map_err(map_error)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let content = to_string_pretty(self).expect("json stringify");
        std::fs::write(path, content).expect("write stuff");
    }

    pub fn update(&mut self) {
        let mut by_path: Vec<(String, i32)> = self
            .requests
            .iter()
            .fold(HashMap::new(), |mut map, req| {
                map.entry(req.path.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                map
            })
            .into_iter()
            .collect();
        by_path.sort_by_key(|(_, n)| -n);
        self.by_path = by_path;

        let mut by_day: Vec<(String, i32)> = self
            .requests
            .iter()
            .fold(HashMap::new(), |mut map, req| {
                let day = req.timestamp.format("%Y-%m-%d").to_string();
                map.entry(day).and_modify(|e| *e += 1).or_insert(1);
                map
            })
            .into_iter()
            .collect();
        by_day.sort_by_key(|(day, _)| day.clone());
        self.by_day = by_day;

        let mut unique_by_day: Vec<(String, i32)> = self
            .requests
            .iter()
            .fold(
                HashMap::new(),
                |mut map: HashMap<String, HashSet<IpAddr>>, req| {
                    let day = req.timestamp.format("%Y-%m-%d").to_string();
                    let entry = map.entry(day).or_insert_with(HashSet::new);
                    entry.insert(req.ip);
                    map
                },
            )
            .into_iter()
            .map(|(day, ips)| (day, ips.len() as i32))
            .collect();
        unique_by_day.sort_by_key(|(day, _)| day.clone());
        self.unique_by_day = unique_by_day;
    }
}

lazy_static! {
    static ref START_TIME: DateTime<FixedOffset> =
        DateTime::parse_from_rfc3339("1900-01-01T00:00:00+00:00").expect("START_TIME");
}

impl Extend<Request> for RequestStats {
    fn extend<T: IntoIterator<Item = Request>>(&mut self, iter: T) {
        let newest = self
            .requests
            .iter()
            .last()
            .map(|req| req.timestamp)
            .unwrap_or(*START_TIME);
        let mut new_requests: Vec<Request> =
            iter.into_iter().filter(|ea| ea.timestamp > newest).collect();
        new_requests.sort_by_key(|ea| ea.timestamp);
        println!("adding {} new log entries", new_requests.len());
        self.requests.extend(new_requests);
        self.update();
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

mod date_serialization {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ts = date.to_rfc3339();
        serializer.serialize_str(&ts)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ts = String::deserialize(deserializer)?;
        let dt = DateTime::parse_from_rfc3339(&ts).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}
