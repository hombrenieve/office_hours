import unittest
import json
from officehours import logAnalyzer

defaultDay="2020/04/20"
defaultStartTime="08:00"
defaultEndTime="17:30"
defaultTotal="09:30"
defaultWorking="08:30"
defaultResting="01:00"
defaultCommand="Start"

class LogBuilder:
    def __init__(self):
        self.log = []
    
    def appendLine(self, day=defaultDay, time=defaultStartTime, command=defaultCommand):
        self.log.append(day+"-"+time+" "+command+"\n")
        return self
    
    def appendStartLine(self, time=defaultStartTime, day=defaultDay):
        self.appendLine(defaultDay, time, "Start")
        return self

    def appendEndLine(self, time=defaultEndTime, day=defaultDay):
        self.appendLine(defaultDay, time, "Stop")
        return self

    def appendPause(self, start, end, day=defaultDay):
        self.appendLine(day, start, "Lock")
        self.appendLine(day, end, "Unlock")
        return self

    def build(self):
        return self.log

class ReportBuilder:
    def __init__(self):
        self.data = { "start": defaultStartTime,
            "end": defaultEndTime,
            "total": defaultTotal,
            "working": defaultWorking,
            "resting": defaultResting }

    def start(self, parameter):
        self.data["start"] = parameter
        return self
    
    def end(self, parameter):
        self.data["end"] = parameter
        return self
    
    def working(self, parameter):
        self.data["working"] = parameter
        return self
    
    def total(self, parameter):
        self.data["total"] = parameter
        return self
    
    def resting(self, parameter):
        self.data["resting"] = parameter
        return self

    def build(self):
        return json.dumps(self.data)


class TestLogAnalyzer(unittest.TestCase):

    def test_report_isEmptyInput(self):
        self.assertEqual(logAnalyzer.report([]), "{}", "Should be empty json string")

    def test_report_noPauses(self):
        logs = LogBuilder().appendStartLine().appendEndLine().build()
        report = ReportBuilder().working("09:30").resting("00:00").build()
        self.assertEqual(logAnalyzer.report(logs), report)
        

if __name__ == '__main__':
    unittest.main()