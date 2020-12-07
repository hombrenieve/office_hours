package model

import "time"

// Report represents a report of the working time
// it shows the real office hours consumed during a session
type Report struct {
	Start   time.Time
	End     time.Time
	Total   time.Duration
	Working time.Duration
	Resting time.Duration
}

func newReport(events []event) *Report {
	report := new(Report)
	report.Start = events[0].moment
	report.End = events[len(events)-1].moment
	report.Total = report.End.Sub(report.Start)
	for ind, event := range events {
		if ind == 0 {
			continue
		}
		if event.prType == eventStop {
			report.Working += event.moment.Sub(events[ind-1].moment)
		}
		if event.prType == eventStart {
			report.Resting += event.moment.Sub(events[ind-1].moment)
		}
	}
	return report
}
