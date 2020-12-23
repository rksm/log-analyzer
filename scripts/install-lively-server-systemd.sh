#!/usr/bin/env bash

if [ -f /etc/systemd/system/lively-log-parser.service ]; then
    sudo cp systemd/admin-worker.service /etc/systemd/system/lively-log-parser.service;
    sudo systemctl daemon-reload
    sudo systemctl restart lively-log-parser.service
else
    sudo cp systemd/lively-log-parser.service /etc/systemd/system/lively-log-parser.service;
    sudo systemctl enable lively-log-parser.service;
    sudo systemctl start lively-log-parser.service;
fi

sudo systemctl status lively-log-parser.service
