package model

import "time"

// Session represents a working hours day
// from the beggining of the labour day till the end
// with all the moments that are an interruption
type Session interface {
	Start(t time.Time)
	Stop(t time.Time)

	Report() (*Report, error)
}

type errorString struct {
	s string
}

func (e *errorString) Error() string {
	return e.s
}

type session struct {
	events []event
}

func (s *session) Start(t time.Time) {
	s.events = append(s.events, event{eventStart, t})
}

func (s *session) Stop(t time.Time) {
	s.events = append(s.events, event{eventStop, t})
}

func (s *session) Report() (*Report, error) {
	if s.events[len(s.events)-1].prType != eventStop {
		return nil, &errorString{"No stop time registered"}
	}
	return newReport(s.events), nil
}

// NewSession creates a new session and initializes it at
// the given time
func NewSession(t time.Time) Session {
	s := new(session)
	s.Start(t)
	return s
}
