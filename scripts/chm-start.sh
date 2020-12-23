#!/usr/bin/env bash

dir=/home/lively/lively-docker/LivelyKernel/log-stats
mkdir -p $dir

./target/x86_64-unknown-linux-musl/release/chm-logs \
      --stats-file $dir/2020-12-22_stats.json \
      --by-day-chart $dir/2020-12-22_by-day.svg \
      --log-dir /var/log/nginx/ \
      --start-date 2020-12-01 \
      --repeat 30
