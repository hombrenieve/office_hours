#!/bin/bash

path="$(dirname $0)"
name=officehours/dbusSessionLogger.py
command="${path}/${name}"
python $command -f ~/.officeHours/$(date +%d-%m-%Y).log &> /dev/null &
disown