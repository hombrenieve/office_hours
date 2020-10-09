use chrono::{Local, DateTime, Duration, TimeZone};

pub mod session {
    use super::*;
    type Moment = chrono::DateTime<chrono::Local>;
    #[cfg(test)]
    use mock_time::now;


    #[derive(Debug)]
    pub struct Session {
        start: Moment,
        end: Option<Moment>,
        events: Vec<Event>,
    }

    #[derive(Eq, PartialEq, Debug)]
    pub struct Report {
        pub start: Option<Moment>, //Will always have a content
        pub end: Option<Moment>,
        pub total: Duration,
        pub working: Duration,
        pub resting: Duration
    }

    #[derive(Debug, Clone)]
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
            self.end.unwrap_or_else(now).signed_duration_since(self.start)
        }

        fn fake_close(&self) -> Vec<Event> {
            let mut events = self.events.to_vec();
            events.push(Event::Close(now()));
            events
        }

        fn get_working(&self) -> Duration {
            let mut previous: Moment = Local.timestamp(0, 0);
            let mut working = self.events.iter().fold(Duration::zero(), |acc, event| {
                match event {
                    Event::Lock(moment) | Event::Close(moment) =>
                        acc + moment.signed_duration_since(previous),
                    Event::Unlock(moment) | Event::Create(moment) => {
                        previous = *moment;
                        acc
                    }
                }
            });
            if self.end == None {
                working = working + now().signed_duration_since(previous);
            }
            working
        }

        pub fn is_session_running(&self) -> bool {
            self.end == None
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

#[cfg(not(test))]
fn now() -> DateTime<Local> {
    Local::now()
}

#[cfg(test)]
pub mod mock_time {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<Option<DateTime<Local>>> = RefCell::new(None);
    }

    pub fn now() -> DateTime<Local> {
        MOCK_TIME.with(|cell| {
            cell.borrow()
                .as_ref()
                .cloned()
                .unwrap_or_else(Local::now)
        })
    }

    pub fn set_mock_time(time: DateTime<Local>) {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = Some(time));
    }

    pub fn clear_mock_time() {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = None);
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    type DTime = DateTime<Local>;
    use session::*;

    fn from_hour(hour: u32, minute: u32) -> DTime {
        Local.ymd(2017, 1, 17).and_hms(hour,minute, 0)
    }

    fn from_hour_next_day(hour: u32, minute: u32) -> DTime {
        Local.ymd(2017, 1, 18).and_hms(hour,minute, 0)
    }

    fn set_system_time(hour: u32, minute: u32) {
        mock_time::set_mock_time(Local.ymd(2017, 1, 17).and_hms(hour,minute, 0));
    }

    struct TestBuilder {
        start: DTime,
        end: Option<DTime>,
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
                total: self.end.unwrap_or_else(mock_time::now)
                    .signed_duration_since(self.start),
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

    #[test]
    fn long_session_no_pause() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            end: Some(from_hour_next_day(8,0)),
            intervals: vec![Event::Close(from_hour_next_day(8,0))],
            working: Duration::hours(24),
            ..TestBuilder::default()
        };
        builder.run_test();
    }

    #[test]
    fn check_current_day() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            working: Duration::hours(6),
            ..TestBuilder::default()
        };
        mock_time::set_mock_time(from_hour(14, 0));
        builder.run_test();
    }

    #[test]
    fn check_current_day_one_pause() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            intervals: vec![
                Event::Lock(from_hour(13,0)),
                Event::Unlock(from_hour(14,0))
            ],
            working: Duration::hours(6),
            resting: Duration::hours(1),

            ..TestBuilder::default()
        };
        mock_time::set_mock_time(from_hour(15, 0));
        builder.run_test();
    }

    #[test]
    fn check_current_day_unfinished_pause() {
        let mut builder = TestBuilder{
            start: from_hour(8,0),
            intervals: vec![
                Event::Lock(from_hour(13,0))
            ],
            working: Duration::hours(5),
            resting: Duration::hours(2),

            ..TestBuilder::default()
        };
        mock_time::set_mock_time(from_hour(15, 0));
        builder.run_test();
    }
}
