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

def _deltaToStr(tdelta):
        hours, rem = divmod(tdelta.seconds, 3600)
        minutes, rem = divmod(rem, 60)
        return "{:02d}:{:02d}".format(hours, minutes)

def _dayHoursToStr(dTime):
    return dTime.strftime("%H:%M")

def _now():
    return datetime.now()

def _proccessHours(prev, curr, condition):
    delta = curr.getTime() - prev.getTime()
    if condition:
        return delta
    else:
        return timedelta()


class Report:

    def __init__(self, lines):
        self._timepoints = list(map(TimePoint, lines))
        self._start = None
        self._end = None
        self._total = None
        self._working = timedelta()
        self._resting = timedelta()
        self._build()

    def _isEmpty(self):
        return len(self._timepoints) == 0

    def _calculateStartTime(self,):
        self._start = self._timepoints[0].getTime()

    def _processWorkingHours(self, prev, curr):
        self._working += _proccessHours(prev, curr, prev.isUnlock())

    def _processRestingHours(self, prev, curr):
        self._resting += _proccessHours(prev, curr, prev.isLock())

    def _calculateOfficeHours(self, processor):
        tp = self._timepoints[0]
        for newTP in self._timepoints[1:]:
            processor(tp, newTP)
            tp = newTP

    def _calculateEndTime(self):
        if self._timepoints[-1].isUnlock():
            #fake timepoint
            tpf = _now()
            delta = tpf - self._timepoints[-1].getTime()
            self._working += delta
            self._end = tpf
        else:
            self._end = self._timepoints[-1].getTime()

    def _calculateTotalTime(self):
        self._total = self._end-self._start
        if self._total.days > 0:
            raise ValueError("Time account exceeds a day")

    def _build(self):
        if self._isEmpty():
            return "{}"

        self._calculateStartTime()
        self._calculateOfficeHours(self._processWorkingHours)
        self._calculateOfficeHours(self._processRestingHours)
        self._calculateEndTime()
        self._calculateTotalTime()

    def report(self):
        return "{}" if self._isEmpty() else json.dumps({
            'start': _dayHoursToStr(self._start),
            'end': _dayHoursToStr(self._end),
            'total': _deltaToStr(self._total),
            'working': _deltaToStr(self._working),
            'resting': _deltaToStr(self._resting)
        })


if __name__ == "__main__":
    with open(sys.argv[1], "r") as infile:
        lines = infile.readlines()
    print(Report(lines).report())