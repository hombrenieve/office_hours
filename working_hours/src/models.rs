pub mod session {
    use std::time::SystemTime;
    type Moment = SystemTime;

    pub struct Session {
        start: Option<Moment>,
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
        pub fn new(event: Event) -> Session {
            if let Event::Create(moment) = event {
                Session {
                    start: Some(moment),
                    end: None,
                    events: vec![Event::Create(moment)]
                }
            } else {
                panic!("Can't use other event than create to initialize session");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use session::*;

    #[test]
    fn check_session_is_correctly_built() {
        let ses = Session::new(Event::Create(SystemTime::now()));
    }

    #[test]
    #[should_panic]
    fn session_cant_be_built_with_other_event() {
        let ses = Session::new(Event::Lock(SystemTime::now()));
    }
}
