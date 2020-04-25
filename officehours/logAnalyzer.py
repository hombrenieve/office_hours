from datetime import datetime,timedelta
import sys
import json

logdatefmt="%Y/%m/%d-%H:%M"

class Report:
    def lineToTimepoint(self, line):
        lineSplit = line.split(" ")
        return (datetime.strptime(lineSplit[0], logdatefmt), 
                lineSplit[1] == "Start" or lineSplit[1] == "Unlock")

    def deltaToStr(self, tdelta):
        hours, rem = divmod(tdelta.seconds, 3600)
        minutes, rem = divmod(rem, 60)
        return "{:02d}:{:02d}".format(hours, minutes)

    def dayHoursToStr(self, dTime):
        return dTime.strftime("%H:%M")

    def now(self):
        return datetime.now()

    def report(self, lines):
        data = {}
        working = timedelta()
        resting = timedelta()

        if(lines != None and len(lines) > 0):
            tp = self.lineToTimepoint(lines[0].rstrip())
            start = tp[0]
            for line in lines[1:]:
                newTP = self.lineToTimepoint(line.rstrip())
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

            data['start'] = self.dayHoursToStr(start)
            data['end'] = self.dayHoursToStr(end)
            total = end-start
            if(total.days > 0):
                raise ValueError("Time account exceeds a day")
            data['total'] = self.deltaToStr(total)
            data['working'] = self.deltaToStr(working)
            data['resting'] = self.deltaToStr(resting)
        return json.dumps(data)

if __name__ == "__main__":
    with open(sys.argv[1], "r") as infile:
        lines = infile.readlines()
    print(Report().report(lines))