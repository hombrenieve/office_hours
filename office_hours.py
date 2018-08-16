#!/usr/bin/env python

import datetime
import os.path
import json
import argparse

def deltaToStr(tdelta):
    hours, rem = divmod(tdelta.seconds, 3600)
    minutes, rem = divmod(rem, 60)
    return "{:02d}:{:02d}".format(hours, minutes)

class OfficeHours:
    LJ=datetime.timedelta(hours=8,minutes=42)
    V=datetime.timedelta(hours=6,minutes=30)
    datefmt="%Y/%m/%d %H:%M"

    def __init__(self, filename=""):
        self._filename = filename
        self._hours = { 'start': None, 'stop': None, 'pauses': [] }
        if os.path.isfile(filename):
            self._load()

    def _load(self):
        with open(self._filename) as json_file:
            self._hours = json.load(json_file)

    def _calcRemainings(self, entry):
        delta=OfficeHours.LJ
        if entry.weekday() == 4:
            delta=OfficeHours.V
        startTime = datetime.datetime.strptime(self._hours['start'], OfficeHours.datefmt)
        exitTime = startTime+delta
        workedTime = entry-startTime
        remainingTime = exitTime-entry
        self._hours['exit'] = exitTime.strftime(OfficeHours.datefmt)
        self._hours['worked'] = deltaToStr(workedTime)
        self._hours['remaining'] = deltaToStr(remainingTime)
        self._hours['total'] = deltaToStr(delta)


    def _writeToFile(self):
        if self._filename != "":
            with open(self._filename, "w") as outfile:
                json.dump(self._hours, outfile)

    def _printInfo(self):
        print json.dumps(self._hours, sort_keys=True, indent=4)

    def start(self, entry=datetime.datetime.now()):
        #Check not started
        if self._hours['start'] != None:
            #it might be an incomplete pause... check!
            pass
        else:
            self._hours['start'] = entry.strftime(OfficeHours.datefmt)
        self._calcRemainings(entry)
        self._writeToFile()
        print("Started, expected exit {}".format(self._hours['exit']))

    def stop(self, entry=datetime.datetime.now()):
        if self._hours['start'] == None:
            print("Error!, you must start first.")
        else:
            self._hours['stop'] = entry.strftime(OfficeHours.datefmt)
            self._calcRemainings(entry)
            self._writeToFile()
            print("Stopped, worked time is {} remaining {}".format(self._hours['worked'], self._hours['remaining']))

    def pause(self, entry=datetime.datetime.now()):
        pass

    def update(self, entry=datetime.datetime.now()):
        if self._hours['start'] == None:
            print("Not started yet!")
        else:
            self._calcRemainings(entry)
            self._writeToFile()
            self._printInfo()


def main(args):
    if args.action == "start":
        OfficeHours(args.filename).start()
    elif args.action == "stop":
        OfficeHours(args.filename).stop()
    elif args.action == "update":
        OfficeHours(args.filename).update()
    elif args.action == "pause":
        OfficeHours(args.filename).pause()
    else:
        print "Undefined action!, choose start/pause/stop"
        
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Record and provide info on working hours for a day")
    parser.add_argument('action', choices=['start', 'pause', 'stop', 'update'], help='The action to take')
    parser.add_argument('-f', default="", dest='filename', help="File to store the info")
    main(parser.parse_args())

