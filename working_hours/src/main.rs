#![feature(proc_macro_hygiene, decl_macro)]

mod models;

pub use models::session::*;

#[macro_use] extern crate rocket;
use rocket::State;
use std::sync::{Arc, Mutex};

struct SessionState {
    sessions: Arc<Mutex<Vec<Session>>>
}

impl SessionState {
    fn new(sessions: Vec<Session>) -> SessionState {
        SessionState {
            sessions: Arc::new(Mutex::new(sessions))
        }
    }

    fn add_session(self, session: Session) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.push(session);
    }

    fn len(&self) -> usize {
        self.sessions.lock().unwrap().len()
    }
}

#[get("/sessions")]
fn get_sessions(session_state: State<SessionState>) -> String {
    format!("The number of sessions is {}", session_state.len())
}

fn rocket(state: SessionState) -> rocket::Rocket {
    rocket::ignite()
        .manage(state)
        .mount("/", routes![get_sessions])
}

fn main() {
    rocket(SessionState::new(vec![])).launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_get_sessions_no_session() {
        let client = Client::new(rocket(SessionState::new(vec![]))).expect("valid rocket instance");
        let mut response = client.get("/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("The number of sessions is 0".into()));
    }

    #[test]
    fn test_get_sessions_one_session() {
        let sessions = vec![Session::new(chrono::Local::now())];
        let client = Client::new(rocket(SessionState::new(sessions))).expect("valid rocket instance");
        let mut response = client.get("/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("The number of sessions is 1".into()));
    }

}
