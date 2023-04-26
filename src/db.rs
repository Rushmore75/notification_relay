use std::env;
use std::time::SystemTime;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{PgConnection, Connection};
use serde::{Deserialize, Serialize};

use crate::authentication::{self, KeyStorage, Session};
use crate::schema::{account, message, read, self};

const REDIS_DATABASE_URL: &'static str = "REDIS_DATABASE_URL";
const POSTGRES_DATABASE_URL: &'static str = "DATABASE_URL";

pub fn redis_connect() -> Result<redis::Connection, redis::RedisError> {
    let url = env::var(REDIS_DATABASE_URL).expect(&format!("{} must be set", REDIS_DATABASE_URL));
    
    let redis = redis::Client::open(url).expect("Can't connect to redis");
    redis.get_connection()
}

pub fn establish_connection() -> PgConnection {

    // the env should be loaded into ram at this point, so there shouldn't be problems running this lots
    let database_url = env::var(POSTGRES_DATABASE_URL).expect(&format!("{} must be set!", POSTGRES_DATABASE_URL));
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//=======================================
//             Account 
//=======================================
#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    email: String,
    password_hash: Vec<u8>,
}

#[derive(Deserialize, Copy, Clone)]
pub struct NewAccount<'a> {
    pub name: &'a str,
    password: &'a str
}

impl Account {
    pub fn new(account: NewAccount<'_>) -> Result<Self, Error> {
        let mut conn = establish_connection(); 
        let hash = authentication::Keyring::<dyn KeyStorage>::hash_string(account.password);

        #[derive(Insertable)]
        #[diesel(table_name = schema::account)]
        struct New<'a> {
            email: &'a str,
            password_hash: Vec<u8>,
        }
        
        let new = New {
            email: account.name,
            password_hash: Vec::from(hash),
        };


        let result = diesel::insert_into(account::table)
            .values(new)
            .get_result(&mut conn);
        result
    }
    
    pub fn get_account_hash(mail: &str) -> Option<Vec<u8>> {
        use crate::schema::account::dsl::*;

        let results: Vec<Self> = account 
            .filter(email.eq(mail))
            .load::<Self>(&mut establish_connection())
            .expect("Error loading accounts");

        match results.into_iter().next() {
            Some(x) => Some(x.password_hash),
            None => None,
        }
    }

    pub fn get_id(session: &Session) -> Option<i32> {
        match Self::get_account(session) {
            Some(e) => Some(e.id),
            None => None,
        }
    }

    pub fn get_account(session: &Session) -> Option<Self> {
        use crate::schema::account::dsl::*;

        let results: Vec<Self> = account 
            .filter(email.eq(session.email.to_owned()))
            .load::<Self>(&mut establish_connection())
            .expect("Error loading accounts");
        // TODO ^^ remove expect

        match results.into_iter().next() {
            Some(x) => Some(x),
            None => None,
        }
    }
}

#[derive(Queryable, Serialize)]
pub struct Message {
    pub id: i32,
    author: i32,
    date: SystemTime,
    content: String,
}

#[derive(Deserialize, Copy, Clone)]
pub struct NewMessage<'a> {
    pub author: i32,
    pub content: &'a str,
}

impl Message {
    pub fn new(message: NewMessage<'_>) -> Result<Self, Error> {
        let mut conn = establish_connection(); 

        #[derive(Insertable)]
        #[diesel(table_name = schema::message)]
        struct New<'a> {
            author: i32,
            content: &'a str,
        }

        let new = New {
            author: message.author,
            content: message.content
        };

        let result = diesel::insert_into(message::table)
            .values(new)
            .get_result(&mut conn);
       result
    }

    pub fn rm(input_id: i32) {
        use crate::schema::message::dsl::*;
        // diesel::delete(users.filter(id.eq(1))).execute(connection)?;

        let result = diesel::delete(
            message.filter(id.eq(input_id)))
            .execute(&mut establish_connection()); 
            
        if result.is_err() {
            println!();
        }

    }
    
    /// This is the primary key. Don't feel bad about using this function.
    pub fn get_by_id(input_id: i32) -> Result<Self, Error> {
        use crate::schema::message::dsl::*;
        
        let result = message
            .filter(id.eq(input_id))
            .first(&mut establish_connection());

        result
    }
    
    /// Get all where `id != input_id`
    pub fn get_all() -> Result<Vec<Self>, Error> {
        use crate::schema::message::dsl::*;
        
        let result = message
            .load(&mut establish_connection());
        result
    }

    pub fn get_all_unread(acc: &Account) -> Result<Vec<i32>, Error> {
        
        let results = message::dsl::message
            .inner_join(
                read::dsl::read.on(
                    message::dsl::id.ne(read::dsl::message)
                    // TODO needs a clause so that messages read by other accounts don't interfere
                )
            )
            // TODO find a way to select all of account
            .select(message::dsl::id)
            .load(&mut establish_connection());

        results
    }
}

#[derive(Queryable)]
pub struct Read {
    id: i32,
    account: i32,
    message: i32,
}


impl Read {
    pub fn new(acc: &Account, msg: &Message) -> Result<Self, Error> {
        let mut conn = establish_connection(); 

        #[derive(Insertable)]
        #[diesel(table_name = schema::read)]
        struct New {
            account: i32,
            message: i32,
        }
        
        let new = New {
            account: acc.id,
            message: msg.id
        };

        let result = diesel::insert_into(read::table)
            .values(new)
            .get_result(&mut conn);
       result
    }
    
    pub fn rm(message_id: &Message, acc: &Account) {
        use crate::schema::read::dsl::*;
        // diesel::delete(users.filter(id.eq(1))).execute(connection)?;

        let result = diesel::delete(read
            .filter(message.eq(message_id.id)))
            .filter(account.eq(acc.id))
            .execute(&mut establish_connection()); 

        if result.is_err(){
            println!();
        }    

    }
    
    

       
}



