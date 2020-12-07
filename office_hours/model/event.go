package model

import "time"

type eventType int

const (
	eventStart eventType = iota
	eventStop
)

type event struct {
	prType eventType
	moment time.Time
}
