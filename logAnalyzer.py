from datetime import datetime,timedelta
import sys

logdatefmt="%Y/%m/%d-%H:%M"

def lineToTimepoint(line):
    lineSplit = line.split(" ")
    return (datetime.strptime(lineSplit[0], logdatefmt), lineSplit[1])

def deltaToStr(tdelta):
    hours, rem = divmod(tdelta.seconds, 3600)
    minutes, rem = divmod(rem, 60)
    return "{:02d}:{:02d}".format(hours, minutes)

def dayHoursToStr(dTime):
    return dTime.strftime("%H:%M")

def main(filename):
    working = timedelta()
    resting = timedelta()
    with open(filename, "r") as infile:
        tp = None
        for line in infile:
            newTP = lineToTimepoint(line.rstrip())
            if tp != None:
                delta = newTP[0]-tp[0]
                if tp[1] == "Start" or tp[1] == "Unlock":
                    working += delta
                else:
                    resting += delta
            else:
                print("Day started at "+dayHoursToStr(newTP[0]))
            tp = newTP
        print("Day ended at "+dayHoursToStr(tp[0]))
    print("Total accounts: ")
    print("    working: "+deltaToStr(working))
    print("    resting: "+deltaToStr(resting))


if __name__ == "__main__":
    main(sys.argv[1])