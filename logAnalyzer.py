from datetime import datetime,timedelta
import sys
import json

logdatefmt="%Y/%m/%d-%H:%M"

def lineToTimepoint(line):
    lineSplit = line.split(" ")
    return (datetime.strptime(lineSplit[0], logdatefmt), 
            lineSplit[1] == "Start" or lineSplit[1] == "Unlock")

def deltaToStr(tdelta):
    hours, rem = divmod(tdelta.seconds, 3600)
    minutes, rem = divmod(rem, 60)
    return "{:02d}:{:02d}".format(hours, minutes)

def dayHoursToStr(dTime):
    return dTime.strftime("%H:%M")

def report(filename):
    data = {}
    working = timedelta()
    resting = timedelta()
    with open(filename, "r") as infile:
        tp = lineToTimepoint(infile.readline().rstrip())
        data['start'] = dayHoursToStr(tp[0])
        for line in infile:
            newTP = lineToTimepoint(line.rstrip())
            delta = newTP[0]-tp[0]
            if tp[1]:
                working += delta
            else:
                resting += delta
            tp = newTP

    if tp[1]:
        #fake timepoint
        tpf = (datetime.now(), False)
        delta = tpf[0] - tp[0]
        working += delta
        tp = tpf

    data['end'] = dayHoursToStr(tp[0])
    data['working'] = deltaToStr(working)
    data['resting'] = deltaToStr(resting)
    return json.dumps(data)


if __name__ == "__main__":
    print report(sys.argv[1])