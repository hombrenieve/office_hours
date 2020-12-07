package model_test

import (
	"office_hours/model"
	"testing"
	"time"
)

func checkReport(t *testing.T, rep *model.Report, err error, total, working, resting string) {
	if rep == nil || err != nil {
		t.Fatalf("Session report should be fine but error %v found\n", err)
	}
	if expectedTotal, _ := time.ParseDuration(total); rep.Total != expectedTotal {
		t.Errorf("Session expected total %q but it is %v\n", total, rep.Total)
	}
	if expectedWorking, _ := time.ParseDuration(working); rep.Working != expectedWorking {
		t.Errorf("Session expected working %q but it is %v\n", working, rep.Working)
	}
	if expectedResting, _ := time.ParseDuration(resting); rep.Resting != expectedResting {
		t.Errorf("Session expected resting %q but it is %v\n", resting, rep.Resting)
	}
}

func TestSessionCreation(t *testing.T) {
	session := model.NewSession(time.Date(2017, time.January, 17, 3, 00, 0, 0, time.UTC))
	if session == nil {
		t.Error("Session creation returned nil object")
	}
}

func TestSessionNoEnd(t *testing.T) {
	session := model.NewSession(time.Date(2017, time.January, 17, 3, 00, 0, 0, time.UTC))
	_, err := session.Report()
	if err == nil {
		t.Error("Session intermediate report should report error")
	}
}

func TestSessionNoPause(t *testing.T) {
	session := model.NewSession(time.Date(2017, time.January, 17, 3, 00, 0, 0, time.UTC))
	session.Stop(time.Date(2017, time.January, 17, 11, 00, 0, 0, time.UTC))
	rep, err := session.Report()
	checkReport(t, rep, err, "8h", "8h", "0s")
}

func TestSessionOnePause(t *testing.T) {
	session := model.NewSession(time.Date(2017, time.January, 17, 8, 00, 0, 0, time.UTC))
	session.Stop(time.Date(2017, time.January, 17, 11, 00, 0, 0, time.UTC))
	session.Start(time.Date(2017, time.January, 17, 12, 00, 0, 0, time.UTC))
	session.Stop(time.Date(2017, time.January, 17, 16, 00, 0, 0, time.UTC))
	rep, err := session.Report()
	checkReport(t, rep, err, "8h", "7h", "1h")
}

func TestSessionSeveralPauses(t *testing.T) {
	session := model.NewSession(time.Date(2017, time.January, 17, 8, 00, 0, 0, time.UTC))

	session.Stop(time.Date(2017, time.January, 17, 9, 00, 0, 0, time.UTC))
	session.Start(time.Date(2017, time.January, 17, 9, 20, 0, 0, time.UTC))

	session.Stop(time.Date(2017, time.January, 17, 13, 00, 0, 0, time.UTC))
	session.Start(time.Date(2017, time.January, 17, 14, 00, 0, 0, time.UTC))

	session.Stop(time.Date(2017, time.January, 17, 16, 00, 0, 0, time.UTC))
	session.Start(time.Date(2017, time.January, 17, 16, 15, 0, 0, time.UTC))

	session.Stop(time.Date(2017, time.January, 17, 17, 30, 0, 0, time.UTC))
	rep, err := session.Report()
	checkReport(t, rep, err, "9h30m", "7h55m", "1h35m")
}
