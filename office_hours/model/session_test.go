package model

import (
	"testing"
	"time"
)

func TestSessionCreation(t *testing.T) {
	session := NewSession(time.Date(2017, time.January, 17, 03, 00, 0, 0, time.UTC))
	if session == nil {
		t.Error("Session creation returned nil object")
	}
}
