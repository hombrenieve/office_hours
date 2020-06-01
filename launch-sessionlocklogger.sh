#!/bin/bash
set -x
path="$(dirname $0)"
name=officehours/dbusSessionLogger.py
command="${path}/${name}"
python3 $command -f ~/.officeHours/$(date +%d-%m-%Y).log &> /dev/null &
disown
