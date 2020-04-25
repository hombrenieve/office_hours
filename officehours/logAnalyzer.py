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


class Report:

    def __init__(self, lines):
        self._timepointIterator = map(TimePoint, lines)
        self._start = None
        self._end = None
        self._working = timedelta()
        self._resting = timedelta()

    def _deltaToStr(self, tdelta):
        hours, rem = divmod(tdelta.seconds, 3600)
        minutes, rem = divmod(rem, 60)
        return "{:02d}:{:02d}".format(hours, minutes)

    def _dayHoursToStr(self, dTime):
        return dTime.strftime("%H:%M")

    def now(self):
        return datetime.now()

    def report(self):
        try:
            tp = next(self._timepointIterator)
            self._start = tp.getTime()
            for newTP in self._timepointIterator:
                delta = newTP.getTime()-tp.getTime()
                if tp.isUnlock():
                    self._working += delta
                else:
                    self._resting += delta
                tp = newTP

            if tp.isUnlock():
                #fake timepoint
                tpf = self.now()
                delta = tpf - tp.getTime()
                self._working += delta
                self._end = tpf
            else:
                self._end = tp.getTime()
        except StopIteration:
            return "{}"

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