from datetime import datetime,timedelta
import sys
import json

logdatefmt="%Y/%m/%d-%H:%M"

class Report:

    def __init__(self, lines):
        self._lines = lines

    def _lineToTimepoint(self, line):
        lineSplit = line.split(" ")
        return (datetime.strptime(lineSplit[0], logdatefmt), 
                lineSplit[1] == "Start" or lineSplit[1] == "Unlock")

    def _deltaToStr(self, tdelta):
        hours, rem = divmod(tdelta.seconds, 3600)
        minutes, rem = divmod(rem, 60)
        return "{:02d}:{:02d}".format(hours, minutes)

    def _dayHoursToStr(self, dTime):
        return dTime.strftime("%H:%M")

    def now(self):
        return datetime.now()

    def report(self):
        data = {}
        working = timedelta()
        resting = timedelta()

        if(self._lines != None and len(self._lines) > 0):
            tp = self._lineToTimepoint(self._lines[0].rstrip())
            start = tp[0]
            for line in self._lines[1:]:
                newTP = self._lineToTimepoint(line.rstrip())
                delta = newTP[0]-tp[0]
                if tp[1]:
                    working += delta
                else:
                    resting += delta
                tp = newTP

            if tp[1]:
                #fake timepoint
                tpf = self.now()
                delta = tpf - tp[0]
                working += delta
                end = tpf
            else:
                end = tp[0]

            data['start'] = self._dayHoursToStr(start)
            data['end'] = self._dayHoursToStr(end)
            total = end-start
            if(total.days > 0):
                raise ValueError("Time account exceeds a day")
            data['total'] = self._deltaToStr(total)
            data['working'] = self._deltaToStr(working)
            data['resting'] = self._deltaToStr(resting)
        return json.dumps(data)

if __name__ == "__main__":
    with open(sys.argv[1], "r") as infile:
        lines = infile.readlines()
    print(Report(lines).report())