
use rocket::{get, response::status, http::{Cookie, CookieJar, Status}, State, post, serde::json::Json, delete};

use crate::{authentication::{Session, SESSION_COOKIE_ID}, db::{NewAccount, Account, Message, NewMessage, Read}};

#[get("/login")]
pub fn login(_auth: Session) -> status::Accepted<&'static str> {
    status::Accepted(Some("Logged in"))
}

#[get("/logout")]
pub async fn logout(auth: Session, keyring: &State<crate::ManagedState>, jar: &CookieJar<'_>) -> status::Accepted<&'static str> {
    keyring.write().await.logout(&auth);
    jar.remove_private(Cookie::named(SESSION_COOKIE_ID));
    status::Accepted(Some("logged out"))
}


#[post("/create_account", data="<body>")]
pub fn create_account(body: Json<NewAccount>) -> status::Custom<String> {
    // TODO needs a good account approval method
    match Account::new(body.0) {
        Ok(_) => status::Custom(Status::Accepted, "Created".to_owned()),
        Err(e) => {
            match e {
                diesel::result::Error::DatabaseError(error_kind, _) => {
                    match error_kind {
                        diesel::result::DatabaseErrorKind::UniqueViolation => {
                            return status::Custom(Status::Conflict, format!("\"{}\" is taken.", body.name));
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
            status::Custom(Status::InternalServerError, "Well heck.".to_owned())
        },
    }
}

#[post("/push_notification", data="<body>")]
pub fn push_notification(auth: Session, body: Json<String>) -> status::Custom<String> {
    match Account::get_id(&auth) {
        Some(e) => {
            match Message::new(NewMessage { author: e, content: body.as_str()}) {
                Ok(e) => {
                    status::Custom(Status::Accepted, format!("Sent, id: {}", e.id))
                },
                Err(e) => {
                    println!("{:?}", e);
                    status::Custom(Status::InternalServerError, "Failed to send message.".to_owned())
                },
            }
        },
        None => status::Custom(Status::BadRequest, "Session is not recognized.".to_owned()),
    }
}

#[delete("/destroy_message/<id>")]
pub fn destroy_notification(_auth: Session, id: i32) {
    Message::rm(id)
}


#[get("/get_all")]
pub fn get_all(auth: Session) -> Json<Vec<Message>> {
    if let Ok(msg) = Message::get_all() {
        // Read::new(&account, &msg);
        return Json::from(msg);        
    }

    Json::from(Vec::<Message>::new()) 
   }

#[post("/read/<input>")]
pub fn mark_read(auth: Session, input: i32) -> status::Custom<&'static str>{
    if let Some(account) = Account::get_account(&auth) {
        if let Ok(msg) = Message::get_by_id(input) {
            match Read::new(&account, &msg) {
                Ok(_) => return status::Custom(Status::Accepted, "Read message."),
                Err(e) => {
                    println!("{:?}", e);
                },
            };
        }
    }
    
    status::Custom(Status::BadRequest, "Cannot read message.\nYou illiterate piece of trash.")
}

#[post("/unread/<input>")]
pub fn mark_unread(auth: Session, input: i32) {

    if let Ok(msg) = Message::get_by_id(input) {
        if let Some(acc) = Account::get_account(&auth) {
            Read::rm(&msg, &acc);
        }
    }

}

#[get("/get_unread")]
pub fn get_all_unread(auth: Session) -> Json<Vec<Message>> {
    if let Some(acc) = Account::get_account(&auth) {
        if let Ok(msg) = Message::get_all_unread(&acc) {
                let messages = msg
                    .iter()
                    .map(|f| Message::get_by_id(*f))
                    .filter(|f| f.is_ok())
                    .map(|f| f.expect("I trusted you! (get_all_unread)"))
                    .collect::<Vec<Message>>();
                return Json::from(messages);
        }
    } 
    
    Json::from(Vec::<Message>::new()) 
}
    
#[get("/version")]
pub fn get_version() -> Json<&'static str> {
    Json::from("0.1.1")
}

