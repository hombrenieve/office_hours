from datetime import datetime,timedelta
import sys
import json

logdatefmt="%Y/%m/%d-%H:%M"

class TimePoint:
    def __init__(self, line):
        lineSplit = line.rstrip().split(" ")
        self._time = datetime.strptime(lineSplit[0], logdatefmt)
        self._command = lineSplit[1]

    def getTime(self):
        return self._time
    
    def isUnlock(self):
        return self._command == "Start" or self._command == "Unlock"
    
    def isLock(self):
        return self._command == "Stop" or self._command == "Lock"

def _calculateStartTime(timepoints):
    return timepoints[0].getTime()

def _processWorkingHours(prev, curr):
    delta = curr.getTime()-prev.getTime()
    if prev.isUnlock():
        return delta
    else:
        return timedelta()

def _processRestingHours(prev, curr):
    delta = curr.getTime()-prev.getTime()
    if prev.isLock():
        return delta
    else:
        return timedelta()

def _calculateOfficeHours(timepoints, processor):
    time = timedelta()
    tp = timepoints[0]
    for newTP in timepoints[1:]:
        time += processor(tp, newTP)
        tp = newTP
    return time

class Report:

    def __init__(self, lines):
        self._timepoints = list(map(TimePoint, lines))

    def _deltaToStr(self, tdelta):
        hours, rem = divmod(tdelta.seconds, 3600)
        minutes, rem = divmod(rem, 60)
        return "{:02d}:{:02d}".format(hours, minutes)

    def _dayHoursToStr(self, dTime):
        return dTime.strftime("%H:%M")

    def now(self):
        return datetime.now()

    def _isEmpty(self):
        return len(self._timepoints) == 0

    def _calculateEndTime(self):
        if self._timepoints[-1].isUnlock():
            #fake timepoint
            tpf = self.now()
            delta = tpf - self._timepoints[-1].getTime()
            self._working += delta
            return tpf
        else:
            return self._timepoints[-1].getTime()

    def report(self):
        if(self._isEmpty()):
            return "{}"

        self._start = _calculateStartTime(self._timepoints)
        self._working = _calculateOfficeHours(self._timepoints, _processWorkingHours)
        self._resting = _calculateOfficeHours(self._timepoints, _processRestingHours)
        self._end = self._calculateEndTime()

        return json.dumps(self._build())

    def _build(self):
        total = self._end-self._start
        if(total.days > 0):
            raise ValueError("Time account exceeds a day")
        return {
            'start': self._dayHoursToStr(self._start),
            'end': self._dayHoursToStr(self._end),
            'total': self._deltaToStr(total),
            'working': self._deltaToStr(self._working),
            'resting': self._deltaToStr(self._resting)
        }

if __name__ == "__main__":
    with open(sys.argv[1], "r") as infile:
        lines = infile.readlines()
    print(Report(lines).report())