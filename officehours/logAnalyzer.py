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
        self._lines = lines

    def _deltaToStr(self, tdelta):
        hours, rem = divmod(tdelta.seconds, 3600)
        minutes, rem = divmod(rem, 60)
        return "{:02d}:{:02d}".format(hours, minutes)

    def _dayHoursToStr(self, dTime):
        return dTime.strftime("%H:%M")

    def now(self):
        return datetime.now()

    def report(self):
        working = timedelta()
        resting = timedelta()

        if(self._lines != None and len(self._lines) > 0):
            tp = TimePoint(self._lines[0])
            start = tp.getTime()
            for line in self._lines[1:]:
                newTP = TimePoint(line)
                delta = newTP.getTime()-tp.getTime()
                if tp.isUnlock():
                    working += delta
                else:
                    resting += delta
                tp = newTP

            if tp.isUnlock():
                #fake timepoint
                tpf = self.now()
                delta = tpf - tp.getTime()
                working += delta
                end = tpf
            else:
                end = tp.getTime()
        else:
            return "{}"

        return json.dumps(self._build(start, end, working, resting))

    def _build(self, start, end, working, resting):
        total = end-start
        if(total.days > 0):
            raise ValueError("Time account exceeds a day")
        return {
            'start': self._dayHoursToStr(start),
            'end': self._dayHoursToStr(end),
            'total': self._deltaToStr(total),
            'working': self._deltaToStr(working),
            'resting': self._deltaToStr(resting)
        }

if __name__ == "__main__":
    with open(sys.argv[1], "r") as infile:
        lines = infile.readlines()
    print(Report(lines).report())