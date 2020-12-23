#!/usr/bin/env bash

cargo run -- \
      --stats-file 2020-12-22_stats.2.json \
      --by-day-chart 2020-12-22_by-day.svg \
      --log-dir ../chm-logs/ \
      --start-date 2020-12-01
