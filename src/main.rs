#![feature(trait_alias)]

mod db;
mod pages;
mod schema;
mod authentication;

use authentication::Keyring;
use dotenvy::dotenv;
use rocket::{routes, tokio::sync::RwLock};


/// Because not everything works with generics... I'm looking at you tokio!
pub type ManagedState = RwLock<Keyring<redis::Connection>>;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    dotenv().ok();
    
    println!("Using Redis!");
    let state = RwLock::new(Keyring { ring: Box::new(db::redis_connect().unwrap()) } );
    let _rocket = rocket::build()
        .manage(state)
        .mount("/", routes![
            pages::login,
            pages::logout,
            pages::create_account,
            pages::push_notification,
            pages::destroy_notification,
            pages::get_all,
            pages::mark_read,
            pages::mark_unread,
            pages::get_all_unread,
            pages::get_version,
            ])
        .launch()
        .await?;
    Ok(())
}
