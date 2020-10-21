#![feature(proc_macro_hygiene, decl_macro)]

mod models;

pub use models::session::*;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
use rocket::State;
use std::sync::Mutex;
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::status::BadRequest;
use std::sync::atomic::{AtomicUsize, Ordering};

type ID = usize;
type SessionMap = HashMap<ID, Session>;

struct SessionState {
    last_id: AtomicUsize, //Nasty trick to dispose once we have DB backend
    sessions: Mutex<SessionMap>
}

impl SessionState {
    fn new(sessions: SessionMap) -> SessionState {
        SessionState {
            last_id: AtomicUsize::new(0),
            sessions: Mutex::new(sessions)
        }
    }

    fn add_session(&self, session: Session) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(self.last_id.fetch_add(1, Ordering::SeqCst), session);
    }

    fn delete_session(&self, id: ID) -> bool {
        match self.sessions.lock().unwrap().remove(&id) {
            Some(_) => true,
            _ => false
        }
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

#[get("/sessions/<id>/report")]
fn get_session_report(id: ID, session_state: State<SessionState>) -> Option<Json<Report>> {
    if let Some(session) = session_state.sessions.lock().unwrap().get(&id) {
        Some(Json(session.get_report()))
    } else {
        None
    }
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

#[delete("/sessions/<id>")]
fn delete_session(id: ID, session_state: State<SessionState>) -> Option<JsonValue> {
    if session_state.delete_session(id) {
        Some(json!({
            "status": "ok",
            "count" : session_state.len()
        }))
    } else {
        None
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found"
    })
}

fn rocket(state: SessionState) -> rocket::Rocket {
    rocket::ignite()
        .manage(state)
        .mount("/", routes![get_sessions, new_session, get_session_report, delete_session])
        .register(catchers![not_found])
}

fn main() {
    rocket(SessionState::new(SessionMap::new())).launch();
}

//#########################################################################

#[cfg(test)] #[macro_use]
extern crate assert_matches;
mod test {
    use super::rocket;
    use super::*;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};

    #[test]
    fn test_get_sessions_no_session() {
        let client = Client::new(rocket(SessionState::new(SessionMap::new()))).expect("valid rocket instance");
        let mut response = client.get("/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"count":0,"status":"ok"}"#.into()));
    }

    #[test]
    fn test_get_sessions_one_session() {
        let session_state = SessionState::new(SessionMap::new());
        session_state.add_session(Session::new(chrono::Local::now()));
        let client = Client::new(rocket(session_state)).expect("valid rocket instance");
        let mut response = client.get("/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"count":1,"status":"ok"}"#.into()));
    }

    #[test]
    fn test_post_sessions_new_session_ok() {
        let client = Client::new(rocket(SessionState::new(SessionMap::new()))).expect("valid rocket instance");
        let mut response = client.post("/sessions")
            .header(ContentType::JSON)
            .body(r#"{ "start": "2014-05-02T20:39:57+01:00" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"id":0,"status":"ok"}"#.into()));
    }

    #[test]
    fn test_post_sessions_new_session_err() {
        let client = Client::new(rocket(SessionState::new(SessionMap::new()))).expect("valid rocket instance");
        let mut response = client.post("/sessions")
            .header(ContentType::JSON)
            .body(r#"{ "start": "03:45:00 17/01/2017 CEST" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.body_string(), Some(r#"{"reason":"Unknown date format","status":"error"}"#.into()));
    }

    #[test]
    fn test_delete_session_ok() {
        let session_state = SessionState::new(SessionMap::new());
        session_state.add_session(Session::new(chrono::Local::now()));
        let client = Client::new(rocket(session_state)).expect("valid rocket instance");
        let mut response = client.delete("/sessions/0").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"count":0,"status":"ok"}"#.into()));
    }

    #[test]
    fn test_delete_session_not_found() {
        let client = Client::new(rocket(SessionState::new(SessionMap::new()))).expect("valid rocket instance");
        let mut response = client.delete("/sessions/3").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.body_string(), Some(r#"{"reason":"Resource was not found","status":"error"}"#.into()));
    }

    #[test]
    fn test_get_session_report_existing_session() {
        let session_state = SessionState::new(SessionMap::new());
        session_state.add_session(Session::new(chrono::Local::now()));
        let client = Client::new(rocket(session_state)).expect("valid rocket instance");
        let mut response = client.get("/sessions/0/report").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_matches!(response.body_string(), Some(_)); //Create regular expression
    }

    #[test]
    fn test_get_session_report_non_existing_session() {
        let session_state = SessionState::new(SessionMap::new());
        session_state.add_session(Session::new(chrono::Local::now()));
        let client = Client::new(rocket(session_state)).expect("valid rocket instance");
        let mut response = client.get("/sessions/2/report").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.body_string(), Some(r#"{"reason":"Resource was not found","status":"error"}"#.into()));
    }
}
