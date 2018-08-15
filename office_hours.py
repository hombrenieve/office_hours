#!/usr/bin/env python

import datetime
import sys
import os.path
import json

class OfficeHours:
    LJ=datetime.timedelta(hours=8,minutes=42)
    V=datetime.timedelta(hours=6,minutes=30)
    datefmt="%Y/%m/%d %H:%M"

    def __init__(self, filename=""):
        self._filename = filename
        self._hours = { 'start': None, 'stop': None, 'pauses': [] }
        if os.path.isfile(filename):
            #would load file
            self._load(filename)

    def _load(self):
        pass

    def _calcRemainings(self, entry):
        delta=OfficeHours.LJ
        if entry.weekday() == 4:
            delta=OfficeHours.V
        startTime = datetime.datetime.strptime(self._hours['start'], OfficeHours.datefmt)
        exitTime = startTime+delta
        workedTime = entry-startTime
        remainingTime = exitTime-entry
        self._hours['exit'] = exitTime.strftime(OfficeHours.datefmt)
        self._hours['worked'] = str(workedTime)
        self._hours['remaining'] = str(remainingTime)
        self._hours['total'] = str(delta)


    def _writeToFile(self):
        if self._filename != "":
            with open(self._filename, "w") as outfile:
                json.dump(self._hours, outfile, sort_keys=True, indent=4)
        else:
            print json.dumps(self._hours, sort_keys=True, indent=4)

    def start(self, entry=datetime.datetime.now()):
        #Check not started
        if self._hours['start'] != None:
            #it might be an incomplete pause... check!
            print("Error!, already started")
        else:
            self._hours['start'] = entry.strftime(OfficeHours.datefmt)
        self._calcRemainings(entry)
        self._writeToFile()

    def stop(self, entry=datetime.datetime.now()):
        #Check not stopped
        if self._hours['stop'] != None:
            print("Error!, already stopped")
        else:
            self._hours['stop'] = entry.strftime(OfficeHours.datefmt)
        self._calcRemainings(entry)
        self._writeToFile()
        pass

    def pause(self, entry=datetime.datetime.now()):
        pass

def main(argv):
    try:
        if argv[1] == "start":
            OfficeHours().start()
        elif argv[1] == "stop":
            OfficeHours().stop()
        elif argv[1] == "pause":
            OfficeHours.pause()
        else:
            print "Undefined action!, choose start/pause/stop"
    except IndexError:
        print "Choose action!, choose start/pause/stop"
        
if __name__ == "__main__":
    main(sys.argv)

