import unittest
from unittest import mock
import json
from datetime import datetime
from officehours import logAnalyzer

defaultDay="2020/04/20"
defaultStartTime="08:00"
defaultEndTime="17:30"
defaultTotal="09:30"
defaultWorking="08:30"
defaultResting="01:00"
defaultCommand="Start"

def defaultEnd():
    return datetime.strptime(defaultDay+" "+defaultEndTime, "%Y/%m/%d %H:%M")

class LogBuilder:
    def __init__(self):
        self.log = []
    
    def newLine(self, time=defaultStartTime, command=defaultCommand,  day=defaultDay):
        self.log.append(day+"-"+time+" "+command+"\n")
        return self
    
    def start(self, time=defaultStartTime, day=defaultDay):
        self.newLine(time, "Start", day)
        return self

    def end(self, time=defaultEndTime, day=defaultDay):
        self.newLine(time, "Stop", day)
        return self

    def pause(self, start, end, day=defaultDay):
        self.newLine(start, "Lock", day)
        self.newLine(end, "Unlock", day)
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
        self.assertEqual(logAnalyzer.Report([]).report(), "{}", "Should be empty json string")

    def test_report_noPauses(self):
        logs = LogBuilder().start().end().build()
        report = ReportBuilder().working("09:30").resting("00:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)
    
    def test_report_onePause(self):
        logs = LogBuilder().start().pause("10:00", "11:00").end().build()
        report = ReportBuilder().working("08:30").resting("01:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)

    def test_report_severalPauses(self):
        logs = LogBuilder().start().pause("10:00", "10:30").pause("13:15", "13:45").end().build()
        report = ReportBuilder().working("08:30").resting("01:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)
    
    def test_report_severalSessionsSameDay(self):
        logs = LogBuilder().start().end("10:00").start("11:00").end().build()
        report = ReportBuilder().working("08:30").resting("01:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)

    def test_report_severalSessionsWithPausesSameDay(self):
        logs = LogBuilder().start().pause("10:00", "10:30").end("10:35").start("11:05").pause("11:10", "11:40").end().build()
        report = ReportBuilder().working("08:00").resting("01:30").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)
    
    def test_report_allDayWorkingWithACrashWhileWorking(self):
        logs = LogBuilder().start().start("11:00").end().build()
        report = ReportBuilder().working("09:30").resting("00:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)
    
    def test_report_allDayWorkingWithPauseWithACrashWhileResting(self):
        logs = LogBuilder().start().newLine("11:00", "Lock").start("12:00").end().build()
        report = ReportBuilder().working("08:30").resting("01:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)

    def test_report_longDayWorking(self):
        logs = LogBuilder().start("09:00", "2020/04/10").end("08:00", "2020/04/11").build()
        report = ReportBuilder().start("09:00").end("08:00").total("23:00").working("23:00").resting("00:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)

    def test_report_moreThanADayWorking(self):
        logs = LogBuilder().start("09:00", "2020/04/10").end("18:00", "2020/04/11").build()
        with self.assertRaises(ValueError) as context:
            logAnalyzer.Report(logs).report()
        self.assertTrue('Time account exceeds a day' in str(context.exception))

    @mock.patch('officehours.logAnalyzer._now', side_effect=defaultEnd)
    def test_checkCurrentDay(self, now_function):
        logs = LogBuilder().start().build()
        report = report = ReportBuilder().working("09:30").resting("00:00").build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)
    
    @mock.patch('officehours.logAnalyzer._now', side_effect=defaultEnd)
    def test_checkCurrentDayWithPause(self, now_function):
        logs = LogBuilder().start().pause("13:00", "14:00").build()
        report = report = ReportBuilder().build()
        self.assertEqual(logAnalyzer.Report(logs).report(), report)

if __name__ == '__main__':
    unittest.main()