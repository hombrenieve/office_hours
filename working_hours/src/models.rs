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
    }

    impl Report {
        pub fn new(session: &Session) -> Report {
            let total = session.get_total();
            let working = session.get_working();
            Report{
                start: Some(session.start),
                end: session.end,
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

    fn calc_duration_since_epoch(minutes: u32) -> (Moment, Moment, Duration) {
        calc_duration(Local.timestamp(0,0), minutes)
    }

    fn calc_duration(origin: Moment, minutes: u32) -> (Moment, Moment, Duration) {
        let duration = Duration::seconds((minutes*60) as i64);
        (origin, origin+duration, duration)
    }

    struct TestBuilder {
        start: Moment,
        end: Moment,
        intervals: Vec<(Moment, Moment)>
    }

    #[test]
    fn session_no_pause() {
        let (origin, end, duration) = calc_duration_since_epoch(60*8);
        let mut sess = Session::new(origin);
        sess.event(Event::Close(end));
        let expected = Report {
            start: Some(origin),
            end: Some(end),
            total: duration,
            working: duration,
            resting: Duration::zero()
        };
        assert_eq!(Report::new(&sess), expected);
    }

    #[test]
    fn session_one_pause() {
        let (start, end, duration) = calc_duration_since_epoch(60*8);
        let (first_pause, end_first_pause, duration_pause) =
        calc_duration(start+Duration::seconds(3600), 60*2);
        let mut sess = Session::new(start);
        sess.event(Event::Lock(first_pause));
        sess.event(Event::Unlock(end_first_pause));
        sess.event(Event::Close(end));
        let expected = Report {
            start: Some(start),
            end: Some(end),
            total: duration,
            working: duration-duration_pause,
            resting: duration_pause
        };
        assert_eq!(Report::new(&sess), expected);
    }

    #[test]
    fn session_two_pauses() {
        let (start, end, duration) = calc_duration_since_epoch(60*8);
        let (first_pause, end_first_pause, duration_pause) =
            calc_duration(start+Duration::seconds(3600), 60*2);
        let (second_pause, end_second_pause, duration_second_pause) =
            calc_duration(start+Duration::seconds(3600*4), 60);
        let mut sess = Session::new(start);
        sess.event(Event::Lock(first_pause));
        sess.event(Event::Unlock(end_first_pause));
        sess.event(Event::Lock(second_pause));
        sess.event(Event::Unlock(end_second_pause));
        sess.event(Event::Close(end));
        let expected = Report {
            start: Some(start),
            end: Some(end),
            total: duration,
            working: duration-duration_pause-duration_second_pause,
            resting: duration_pause+duration_second_pause
        };
        assert_eq!(Report::new(&sess), expected);
    }

    #[test]
    fn session_one_pause_several_locks() {
        let (start, end, duration) = calc_duration_since_epoch(60*8);
        let (first_pause, end_first_pause, duration_pause) =
            calc_duration(start+Duration::seconds(3600), 60*2);
        let mut sess = Session::new(start);
        sess.event(Event::Lock(first_pause));
        sess.event(Event::Lock(first_pause+Duration::seconds(10)));
        sess.event(Event::Lock(first_pause+Duration::seconds(100)));
        sess.event(Event::Unlock(end_first_pause));
        sess.event(Event::Close(end));
        let expected = Report {
            start: Some(start),
            end: Some(end),
            total: duration,
            working: duration-duration_pause,
            resting: duration_pause
        };
        assert_eq!(Report::new(&sess), expected);
    }

    #[test]
    fn session_one_pause_several_unlocks() {
        let (start, end, duration) = calc_duration_since_epoch(60*8);
        let (first_pause, end_first_pause, duration_pause) =
            calc_duration(start+Duration::seconds(3600), 60*2);
        let mut sess = Session::new(start);
        sess.event(Event::Lock(first_pause));
        sess.event(Event::Unlock(end_first_pause));
        sess.event(Event::Unlock(end_first_pause+Duration::seconds(10)));
        sess.event(Event::Unlock(end_first_pause+Duration::seconds(100)));
        sess.event(Event::Close(end));
        let expected = Report {
            start: Some(start),
            end: Some(end),
            total: duration,
            working: duration-duration_pause,
            resting: duration_pause
        };
        assert_eq!(Report::new(&sess), expected);
    }

    #[test]
    fn session_one_pause_several_lock_unlock_same_time() {
        let (start, end, duration) = calc_duration_since_epoch(60*8);
        let (first_pause, end_first_pause, duration_pause) =
            calc_duration(start+Duration::seconds(3600), 60*2);
        let mut sess = Session::new(start);
        sess.event(Event::Lock(first_pause));
        sess.event(Event::Unlock(first_pause));
        sess.event(Event::Lock(first_pause));
        sess.event(Event::Unlock(first_pause));
        sess.event(Event::Lock(first_pause));
        sess.event(Event::Unlock(end_first_pause));
        sess.event(Event::Close(end));
        let expected = Report {
            start: Some(start),
            end: Some(end),
            total: duration,
            working: duration-duration_pause,
            resting: duration_pause
        };
        assert_eq!(Report::new(&sess), expected);
    }

}
