#![feature(proc_macro_hygiene, decl_macro)]

mod models;

pub use models::session::*;

#[macro_use] extern crate rocket;

mod main {
    use super::*;

    pub static SESSIONS: Vec<Session> = vec![];
}

#[get("/sessions")]
fn get_sessions() -> String {
    format!("The number of sessions is {}", main::SESSIONS.len())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![get_sessions])
}

fn main() {
    rocket().launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_get_sessions() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("The number of sessions is 0".into()));
    }
}
