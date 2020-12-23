use chrono::{DateTime, FixedOffset, NaiveDate};

use crate::error::{Error, Result};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Options {
    pub stats_file: PathBuf,
    pub log_dir: PathBuf,
    pub start_date: DateTime<FixedOffset>,
    pub by_day_chart_file: Option<PathBuf>,
}

pub fn parse_args() -> Result<Options> {
    let args: Vec<String> = env::args().collect();
    let prog_name = Path::new(&args[0])
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| Error::CliError("unable to get executable name".to_string()))?;
    let mut opts = getopts::Options::new();
    opts.reqopt("", "stats-file", "where to put the stats", "FILE");
    opts.reqopt("", "log-dir", "where the log files are", "DIR");
    opts.optopt(
        "",
        "by-day-chart",
        "svg file to store the chart into",
        "FILE.svg",
    );
    opts.optopt("", "start-date", "date to start from", "2020-12-23");

    match opts.parse(&args) {
        Err(_) => {
            eprintln!("{}", opts.short_usage(&prog_name));
            Err(Error::CliError("Unable to parse options".to_string()))
        }
        Ok(parsed) => {
            let stats_file: PathBuf = parsed.opt_get("stats-file").unwrap().expect("stats-file");
            let log_dir: PathBuf = parsed.opt_get("log-dir").unwrap().expect("log-dir");
            let start_date = parsed
                .opt_str("start-date")
                .map(|arg| {
                    let d = NaiveDate::parse_from_str(&arg, "%Y-%m-%d").unwrap();
                    let dt = d.and_hms(0, 0, 0);
                    DateTime::<FixedOffset>::from_utc(dt, FixedOffset::east(0))
                })
                .unwrap_or_else(|| {
                    DateTime::parse_from_rfc3339("2020-01-01T00:00:00+00:00").unwrap()
                });

            let by_day_chart_file = parsed.opt_str("by-day-chart").map(PathBuf::from);

            Ok(Options {
                stats_file,
                log_dir,
                start_date,
                by_day_chart_file,
            })
        }
    }
}
