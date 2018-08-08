#!/usr/bin/env python

import datetime
import sys

LJ=datetime.timedelta(hours=8,minutes=42)
V=datetime.timedelta(hours=6,minutes=30)

entry=datetime.datetime.now()
if len(sys.argv) == 2:
   entrytime=datetime.datetime.strptime(sys.argv[1], "%H:%M")
   entry=entry.replace(hour=entrytime.hour, minute=entrytime.minute)

delta=LJ
if entry.weekday() == 4:
   delta=V

print (entry+delta).strftime("%H:%M")
