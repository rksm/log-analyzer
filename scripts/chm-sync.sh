#!/usr/bin/env bash

TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

ssh lively-chm "mkdir -p log-parser"

rsync -avz -P --delete --delete-excluded \
      --filter=". ./scripts/chm-file-filter.txt" \
      ./ lively-chm:log-parser/


ssh lively-chm 'bash -c "cd log-parser; ./scripts/chm-install-systemd.sh"'
