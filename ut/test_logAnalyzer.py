import unittest
from officehours import logAnalyzer

class TestLogAnalyzer(unittest.TestCase):

    def test_report_isEmptyInput(self):
        self.assertEqual(logAnalyzer.report([]), "{}", "Should be empty json string")

if __name__ == '__main__':
    unittest.main()