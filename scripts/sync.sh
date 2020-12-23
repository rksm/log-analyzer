#!/usr/bin/env bash

TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

ssh lively-chm "mkdir -p log-parser"

rsync -avz -P --delete \
      --exclude .git \
      ./ lively-chm:log-parser/


ssh lively-chm 'bash -c "log-parser; ./scripts/install-lively-server-systemd.sh"'
