#![allow(dead_code)]

use access_log_parser::*;
use chrono::{DateTime, FixedOffset};
use std::{fmt::Debug, fs::read_dir, fs::read_to_string, fs::DirEntry, path::Path, path::PathBuf};

use crate::stats::Request;

const LOG_DIR: &str = "/Users/robert/projects/lively/chm/2020-12-22-logs/";

pub fn find_log_files() -> Vec<PathBuf> {
    let dir = read_dir(LOG_DIR).expect("read log dir");
    let mut files: Vec<DirEntry> = dir
        .into_iter()
        .filter_map(|ea| ea.ok())
        .filter(|file| {
            return file.file_name().to_string_lossy().starts_with("access.log");
        })
        .collect();

    files.sort_by_key(|ea| {
        ea.metadata()
            .and_then(|meta| meta.created())
            .and_then(|created| {
                created
                    .elapsed()
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
            })
            .unwrap_or_else(|_| std::time::Duration::from_millis(0))
    });

    files.into_iter().map(|ea| ea.path()).collect()
}

fn parse_log_file<P: AsRef<Path> + Debug>(
    path: P,
    until: DateTime<FixedOffset>,
) -> (bool, Vec<Request>) {
    let content = read_to_string(&path).expect("read log file");
    let lines: Vec<_> = content.lines().rev().collect(); // lines are oldest to newest
    let mut result: Vec<Request> = Vec::new();

    for line in lines {
        if let Ok(LogEntry::CombinedLog(entry)) = parse(LogType::CombinedLog, line) {
            if let LogFormatValid::Valid(req) = entry.request {
                if entry.timestamp < until {
                    println!(
                        "stop parsing at timestamp {:#?} in file {:?}",
                        entry.timestamp, path
                    );
                    return (true, result);
                }

                let method = req.method().as_str();
                let is_get = "GET" == method;
                let is_html = req
                    .uri()
                    .path_and_query()
                    .map(|p| {
                        p.path().ends_with(".html")
                            && !p.path().starts_with("/PartsBin/")
                            && !p.path().starts_with("/proxy/")
                    })
                    .unwrap_or(false);

                if !is_get || !is_html {
                    continue;
                }

                let referrer: String = entry
                    .referrer
                    .map(|r| r.path().to_string())
                    .unwrap_or_default();
                let path = req.uri().path_and_query().unwrap().path().to_string();
                let user_agent = entry.user_agent.map(|r| r.into()).unwrap_or_default();
                result.push(Request {
                    ip: entry.ip,
                    method: method.to_string(),
                    path,
                    timestamp: entry.timestamp,
                    status_code: entry.status_code.into(),
                    referrer,
                    user_agent,
                });
            }
        }
    }

    (false, result)
}

pub fn parse_log_files(files: &[PathBuf], stop_date: DateTime<FixedOffset>) -> Vec<Request> {
    let mut parsed: Vec<Request> = Vec::new();
    for file in files {
        println!("parsing {:#?}", file);
        let (stopped, result) = parse_log_file(file, stop_date);
        parsed.extend(result);
        if stopped {
            break;
        }
    }
    parsed.sort_by_key(|ea| ea.timestamp);
    parsed
}
