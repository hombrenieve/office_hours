#![feature(proc_macro_hygiene, decl_macro)]

mod models;

pub use models::session::*;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
use rocket::State;
use std::sync::{Arc, Mutex};
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::status::BadRequest;


struct SessionState {
    sessions: Arc<Mutex<Vec<Session>>>
}

impl SessionState {
    fn new(sessions: Vec<Session>) -> SessionState {
        SessionState {
            sessions: Arc::new(Mutex::new(sessions))
        }
    }

    fn add_session(&self, session: Session) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.push(session);
    }

    fn len(&self) -> usize {
        self.sessions.lock().unwrap().len()
    }
}

//################ BINDINGS ######################

#[get("/sessions")]
fn get_sessions(session_state: State<SessionState>) -> JsonValue {
    json!({
        "status": "ok",
        "count": session_state.len()
    })
}

#[derive(Deserialize)]
struct SessionMessage {
    start: String
}

#[post("/sessions", format = "json", data = "<session>")]
fn new_session(session: Json<SessionMessage>, session_state: State<SessionState>) -> Result<JsonValue, BadRequest<JsonValue>> {
    if let Ok(session_start) = chrono::DateTime::parse_from_rfc3339(&session.0.start) {
        session_state.add_session(Session::new(chrono::DateTime::from(session_start)));
        Ok(json!({
            "status": "ok",
            "id" : session_state.len()-1
        }))
    } else {
        Err(BadRequest(Some(json!({
            "status": "error",
            "reason": "Unknown date format"
        }))))
    }
}

fn rocket(state: SessionState) -> rocket::Rocket {
    rocket::ignite()
        .manage(state)
        .mount("/", routes![get_sessions, new_session])
}

fn main() {
    rocket(SessionState::new(vec![])).launch();
}

//#########################################################################

#[cfg(test)]
mod test {
    use super::rocket;
    use super::*;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};

    #[test]
    fn test_get_sessions_no_session() {
        let client = Client::new(rocket(SessionState::new(vec![]))).expect("valid rocket instance");
        let mut response = client.get("/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"count":0,"status":"ok"}"#.into()));
    }

    #[test]
    fn test_get_sessions_one_session() {
        let sessions = vec![Session::new(chrono::Local::now())];
        let client = Client::new(rocket(SessionState::new(sessions))).expect("valid rocket instance");
        let mut response = client.get("/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"count":1,"status":"ok"}"#.into()));
    }

    #[test]
    fn test_post_sessions_new_session_ok() {
        let client = Client::new(rocket(SessionState::new(vec![]))).expect("valid rocket instance");
        let mut response = client.post("/sessions")
            .header(ContentType::JSON)
            .body(r#"{ "start": "2014-05-02T20:39:57+01:00" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"id":0,"status":"ok"}"#.into()));
    }

    #[test]
    fn test_post_sessions_new_session_err() {
        let client = Client::new(rocket(SessionState::new(vec![]))).expect("valid rocket instance");
        let mut response = client.post("/sessions")
            .header(ContentType::JSON)
            .body(r#"{ "start": "03:45:00 17/01/2017 CEST" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.body_string(), Some(r#"{"reason":"Unknown date format","status":"error"}"#.into()));
    }
}
