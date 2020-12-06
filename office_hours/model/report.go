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
