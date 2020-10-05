pub mod session {
    use std::time::{SystemTime, Duration};
    type Moment = SystemTime;

    pub struct Session {
        start: Moment,
        end: Option<Moment>,
        events: Vec<Event>
    }

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
        pub fn get_total(&self) -> Option<Duration> {
            match self.end {
                Some(end_time) => match end_time.duration_since(self.start) {
                    Ok(elapsed) => Some(elapsed),
                    _ => None
                }
                _ => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, Duration};
    use session::*;

    fn calc_duration(minutes: u32) -> (SystemTime, SystemTime, Duration) {
        let duration = Duration::from_secs((minutes*60) as u64);
        let now = SystemTime::now();
        (now, now+duration, duration)
    }

    #[test]
    fn check_session_is_correctly_built() {
        let ses = Session::new(SystemTime::now());
    }

    #[test]
    fn close_event_collect_total() {
        let (start, end, duration) = calc_duration(90);
        let mut ses = Session::new(start);
        ses.event(Event::Close(end));
        assert_eq!(ses.get_total(), Some(duration));
    }
}
