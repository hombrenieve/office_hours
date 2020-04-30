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

class ReportBuilder:
    def __init__(self, report):
        self.__report = report

    @staticmethod
    def __deltaToStr(tdelta):
        hours, rem = divmod(tdelta.seconds, 3600)
        minutes, rem = divmod(rem, 60)
        return "{:02d}:{:02d}".format(hours, minutes)

    @staticmethod
    def __dayHoursToStr(dTime):
        return dTime.strftime("%H:%M")

    def build(self):
        return {
            'start': ReportBuilder.__dayHoursToStr(self.__report["_Report__start"]),
            'end': ReportBuilder.__dayHoursToStr(self.__report["_Report__end"]),
            'total': ReportBuilder.__deltaToStr(self.__report["_Report__total"]),
            'working': ReportBuilder.__deltaToStr(self.__report["_Report__working"]),
            'resting': ReportBuilder.__deltaToStr(self.__report["_Report__resting"])
        }


class Report:

    def __init__(self, lines):
        self.__timepoints = list(map(TimePoint, lines))
        self.__start = None
        self.__end = None
        self.__total = None
        self.__working = timedelta()
        self.__resting = timedelta()
        try:
            self.__build()
        except IndexError:
            pass

    @staticmethod
    def _now():
        return datetime.now()

    def _isEmpty(self):
        return len(self.__timepoints) == 0

    def _calculateStartTime(self,):
        self.__start = self.__timepoints[0].getTime()

    def _processWorkingHours(self, prev, curr):
        self.__working += Report._proccessHours(prev, curr, prev.isUnlock())

    def _processRestingHours(self, prev, curr):
        self.__resting += Report._proccessHours(prev, curr, prev.isLock())

    @staticmethod
    def _proccessHours(prev, curr, condition):
        delta = curr.getTime() - prev.getTime()
        if condition:
            return delta
        else:
            return timedelta()

    def _calculateOfficeHours(self, processor):
        tp = self.__timepoints[0]
        for newTP in self.__timepoints[1:]:
            processor(tp, newTP)
            tp = newTP

    def _adjustEndTimeWhenItsNotFound(self):
        tpf = Report._now()
        delta = tpf - self.__timepoints[-1].getTime()
        self.__working += delta
        return tpf

    def _calculateEndTime(self):
        self.__end = self.__timepoints[-1].getTime() if self.__timepoints[-1].isLock() else self._adjustEndTimeWhenItsNotFound()

    def _calculateTotalTime(self):
        self.__total = self.__end - self.__start
        if self.__total.days > 0:
            raise ValueError("Time account exceeds a day")

    def __build(self):
        self._calculateStartTime()
        self._calculateOfficeHours(self._processWorkingHours)
        self._calculateOfficeHours(self._processRestingHours)
        self._calculateEndTime()
        self._calculateTotalTime()

    def report(self):
        return "{}" if self._isEmpty() else json.dumps(ReportBuilder(self.__dict__).build())


if __name__ == "__main__":
    with open(sys.argv[1], "r") as infile:
        lines = infile.readlines()
    print(Report(lines).report())