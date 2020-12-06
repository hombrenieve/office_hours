package model

import "time"

// Session represents a working hours day
// from the beggining of the labour day till the end
// with all the moments that are an interruption
type Session interface {
	Start(t time.Time)
	Pause(t time.Time)
	Stop(t time.Time)

	Report() (*Report, error)
}

type errorString struct {
	s string
}

func (e *errorString) Error() string {
	return e.s
}

type eventType int

const (
	eventStart eventType = iota
	eventPause
	eventStop
)

type event struct {
	prType eventType
	moment time.Time
}

type session struct {
	events []event
}

func (s *session) Start(t time.Time) {

}

func (s *session) Stop(t time.Time) {

}

func (s *session) Pause(t time.Time) {

}

func (s *session) Report() (*Report, error) {
	return nil, &errorString{"Generic"}
}

// NewSession creates a new session and initializes it at
// the given time
func NewSession(t time.Time) Session {
	s := new(session)
	s.Start(t)
	return s
}
