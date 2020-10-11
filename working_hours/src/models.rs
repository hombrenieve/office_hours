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

    impl Event {
        fn delta_from(&self, other: &Event) -> (Status, Duration) {
            let status: Status;
            let other_time: &Moment;
            match other {
                Event::Create(moment) | Event::Unlock(moment) => {
                    status = Status::Working;
                    other_time = moment;
                }
                Event::Close(moment) | Event::Lock(moment) => {
                    status = Status::Resting;
                    other_time = moment;
                }
            };
            match self {
                Event::Create(moment) | Event::Unlock(moment) |
                Event::Close(moment) | Event::Lock(moment) => {
                    let duration = moment.signed_duration_since(*other_time);
                    (status, duration)
                }
            }
        }
    }

    #[derive(Eq, PartialEq)]
    enum Status {
        Resting,
        Working
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
            match an_event {
                Event::Close(moment) => {
                    self.end = Some(moment);
                },
                _ => ()
            }
            self.events.push(an_event);
        }

        fn get_total(&self) -> Duration {
            self.end.unwrap_or_else(now).signed_duration_since(self.start)
        }

        fn fake_close(&self) -> Vec<Event> {
            let mut events = self.events.to_vec();
            events.push(Event::Close(now()));
            events
        }

        fn calculate_deltas(events: &Vec<Event>) -> Vec<(Status, Duration)> {
            let mut deltas: Vec<(Status, Duration)> = vec![];
            let mut previous = events.first().unwrap();
            for event in events.iter().skip(1) {
                deltas.push(event.delta_from(previous));
                previous = event;
            }
            deltas
        }

        fn calculate_office_hours(&self) -> (Duration, Duration) {
            let deltas: Vec<(Status, Duration)>;
            if self.end == None {
                deltas = Self::calculate_deltas(&self.fake_close());
            } else {
                deltas = Self::calculate_deltas(&self.events);
            }
            let resting = deltas.iter().filter(|e| e.0 == Status::Resting)
                .map(|e| e.1)
                .fold(Duration::zero(), |acc, e| acc+e);
            let working = deltas.iter().filter(|e| e.0 == Status::Working)
                .map(|e| e.1)
                .fold(Duration::zero(), |acc, e| acc+e);
            (working, resting)
        }

        pub fn is_session_running(&self) -> bool {
            self.end == None
        }

        pub fn get_report(&self) -> Report {
            let total = self.get_total();
            let (working, resting) = self.calculate_office_hours();
            Report{
                start: Some(self.start),
                end: self.end,
                total: total,
                working: working,
                resting: resting
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
