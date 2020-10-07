pub mod session {
    use chrono::{Local, DateTime, Duration, TimeZone};
    type Moment = DateTime<Local>;

    #[derive(Debug)]
    pub struct Session {
        start: Moment,
        end: Option<Moment>,
        events: Vec<Event>
    }

    #[derive(Eq, PartialEq, Debug)]
    pub struct Report {
        pub start: Option<Moment>, //Will always have a content
        pub end: Option<Moment>,
        pub total: Duration,
        pub working: Duration,
        pub resting: Duration
    }

    #[derive(Debug)]
    pub enum Event {
        Create(Moment),
        Lock(Moment),
        Unlock(Moment),
        Close(Moment)
    }

    impl Session {

        pub fn new(moment: Moment) -> Session {
            Session {
                start: moment,
                end: None,
                events: vec![Event::Create(moment)]
            }
        }

        pub fn event(&mut self, an_event: Event) {
            if let Some(last_event) = self.events.last() {
                match an_event {
                    Event::Lock(_) => match last_event {
                        Event::Unlock(_) | Event::Create(_) => self.events.push(an_event),
                        _ => ()
                    },
                    Event::Unlock(_) => match last_event {
                        Event::Lock(_) => self.events.push(an_event),
                        _ => ()
                    },
                    Event::Close(moment) => {
                        self.events.push(an_event);
                        self.end = Some(moment);
                    },
                    _ => ()
                }
            }
        }

        fn get_total(&self) -> Duration {
            match self.events.last() {
                Some(Event::Create(_)) => Duration::zero(),
                Some(event) => match event {
                    Event::Close(moment) | Event::Lock(moment) | Event::Unlock(moment) =>
                        moment.signed_duration_since(self.start),
                    _ => Duration::zero()
                },
                _ => Duration::zero()
            }
        }

        fn get_working(&self) -> Duration {
            let mut previous: Moment = Local.timestamp(0,0);
            self.events.iter().fold(Duration::zero(), | acc, event | {
                match event {
                    Event::Lock(moment) | Event::Close(moment) =>
                        acc+moment.signed_duration_since(previous),
                    Event::Unlock(moment) | Event::Create(moment )=> {
                        previous = *moment;
                        acc
                    }
                }
            })
        }

        pub fn get_report(&self) -> Report {
            let total = self.get_total();
            let working = self.get_working();
            Report{
                start: Some(self.start),
                end: self.end,
                total: total,
                working: working,
                resting: total-working
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, DateTime, Duration, TimeZone};
    type Moment = DateTime<Local>;
    use session::*;

    fn from_hour(hour: u32, minute: u32) -> Moment {
        Local.ymd(2017, 1, 17).and_hms(hour,minute, 0)
    }

    struct TestBuilder {
        start: Moment,
        end: Option<Moment>,
        intervals: Vec<Event>,
        working: Duration,
        resting: Duration
    }

    impl Default for TestBuilder {
        fn default() -> Self {
            TestBuilder{
                start: Local.timestamp(0,0),
                end: None,
                intervals: vec![],
                working: Duration::zero(),
                resting: Duration::zero()
            }
        }
    }

    impl TestBuilder {



        fn build_expected(&self) -> Report {
            Report {
                start: Some(self.start),
                end: self.end,
                total: self.end.unwrap().signed_duration_since(self.start),
                working: self.working,
                resting: self.resting
            }
        }

        fn run_test(&mut self) {
            let mut sess = Session::new(self.start);
            for int in self.intervals.drain(..) {
                sess.event(int);
            }
            assert_eq!(self.build_expected(), sess.get_report());
        }

    }

    #[test]
    fn session_no_pause() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            end: Some(from_hour(16,0)),
            intervals: vec![Event::Close(from_hour(16,0))],
            working: Duration::hours(8),
            ..TestBuilder::default()
        };
        builder.run_test();
    }

    #[test]
    fn session_one_pause() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            end: Some(from_hour(16,0)),
            intervals: vec![
                Event::Lock(from_hour(13,0)),
                Event::Unlock(from_hour(14,0)),
                Event::Close(from_hour(16,0))
            ],
            working: Duration::hours(7),
            resting: Duration::hours(1)
        };
        builder.run_test();
    }

    #[test]
    fn session_two_pauses() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            end: Some(from_hour(16,0)),
            intervals: vec![
                Event::Lock(from_hour(8,30)),
                Event::Unlock(from_hour(9,0)),
                Event::Lock(from_hour(13,0)),
                Event::Unlock(from_hour(14,0)),
                Event::Close(from_hour(16,0))
            ],
            working: Duration::minutes(6*60+30),
            resting: Duration::minutes(60+30)
        };
        builder.run_test();
    }

    #[test]
    fn session_one_pause_several_locks() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            end: Some(from_hour(16,0)),
            intervals: vec![
                Event::Lock(from_hour(13,0)),
                Event::Lock(from_hour(13,10)),
                Event::Lock(from_hour(13,30)),
                Event::Unlock(from_hour(14,0)),
                Event::Close(from_hour(16,0))
            ],
            working: Duration::hours(7),
            resting: Duration::hours(1)
        };
        builder.run_test();
    }

    #[test]
    fn session_one_pause_several_unlocks() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            end: Some(from_hour(16,0)),
            intervals: vec![
                Event::Lock(from_hour(13,0)),
                Event::Unlock(from_hour(14,0)),
                Event::Unlock(from_hour(14,10)),
                Event::Unlock(from_hour(14,30)),
                Event::Close(from_hour(16,0))
            ],
            working: Duration::hours(7),
            resting: Duration::hours(1)
        };
        builder.run_test();
    }

    #[test]
    fn session_one_pause_several_lock_unlock_same_time() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            end: Some(from_hour(16,0)),
            intervals: vec![
                Event::Lock(from_hour(13,0)),
                Event::Unlock(from_hour(13,0)),
                Event::Lock(from_hour(13,0)),
                Event::Unlock(from_hour(13,0)),
                Event::Lock(from_hour(13,0)),
                Event::Unlock(from_hour(14,0)),
                Event::Close(from_hour(16,0))
            ],
            working: Duration::hours(7),
            resting: Duration::hours(1)
        };
        builder.run_test();
    }

}
