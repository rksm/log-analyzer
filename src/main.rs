mod error;
mod options;
mod parser;
mod stats;
mod viz;

use std::path::Path;

use chrono::{DateTime, FixedOffset};
use error::Result;
use options::{parse_args, Options};
use parser::{find_log_files, parse_log_files};
use stats::RequestStats;
use std::thread::sleep;
use viz::draw_pictures;

fn print_stats(stats: &RequestStats) {
    println!("by html GET requests");
    for (path, n_requests) in &stats.by_path {
        println!("{}\t{}", n_requests, path);
    }

    println!("by day");
    for (path, n_requests) in &stats.by_day {
        println!("{}\t{}", n_requests, path);
    }

    println!("unique by day");
    for (path, n_requests) in &stats.unique_by_day {
        println!("{}\t{}", n_requests, path);
    }
}

fn read_stats<P: AsRef<Path> + std::fmt::Debug>(path: P) -> RequestStats {
    match RequestStats::from_file(&path) {
        Err(err) => {
            eprintln!("[error] Cannot read stats from {:?} {:?}", path, err);
            RequestStats::default()
        }
        Ok(stats) => stats,
    }
}

fn main2() {
    let opts = parse_args().unwrap();
    println!("{:#?}", opts);

    let d = chrono::NaiveDate::parse_from_str("2020-12-23", "%Y-%m-%d").unwrap();
    let dt = d.and_hms(0, 0, 0);
    let dt = DateTime::<FixedOffset>::from_utc(dt, chrono::FixedOffset::east(0));
    println!("{:#?}", dt);
}

fn run(opts: &Options) {
    // read existing stats
    println!("Reading stats from {:?}", opts.stats_file);
    let mut stats = read_stats(&opts.stats_file);

    // find the log files
    let files = find_log_files(&opts.log_dir);

    // read from the log files up to what we read before (according to the last
    // timestamp)
    let last_timestamp = stats
        .requests
        .iter()
        .last()
        .map(|req| req.timestamp)
        .unwrap_or(opts.start_date);

    // update the existing stats with the new requests from the logs
    let parsed = parse_log_files(&files, last_timestamp);
    let before = stats.requests.len();
    stats.extend(parsed);
    let after = stats.requests.len();

    // update the stats file
    if after > before {
        println!(
            "Found {} new log items, saving to {:?}",
            after - before,
            opts.stats_file
        );
        stats.save(&opts.stats_file);
    }

    // print the stats
    print_stats(&stats);

    if let Some(chart_file) = &opts.by_day_chart_file {
        draw_pictures(chart_file, &stats);
    }
}

fn main() -> Result<()> {
    let opts = parse_args()?;

    loop {
        run(&opts);

        if let Some(mins) = opts.repeat_mins {
            let duration = std::time::Duration::from_secs(mins * 60);
            println!("waiting {:#?} until next iteration", duration);
            sleep(duration);
        } else {
            break;
        }
    }
    Ok(())
}
