#![allow(dead_code)] // usful in dev mode
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
use actix::prelude::*;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use chrono::Duration;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
mod auth_handler;
mod email_service;
mod errors;
mod invitation_handler;
mod models;
mod register_handler;
mod schema;
mod utils;

use crate::models::DbExecutor;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let sys = actix_rt::System::new("example");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    HttpServer::new(move || {
        // secret is a random minimum 32 bytes long base 64 string
        let secret: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
        let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

        App::new()
            // add database pool as data/state to the app
            .data(address.clone())
            // setup logger for requests
            .wrap(Logger::default())
            // everything under '/api/' route
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            .service(
                web::scope("/api")
                    // routes for authentication
                    .service(
                        web::resource("/auth")
                            .route(web::post().to_async(auth_handler::login))
                            .route(web::delete().to(auth_handler::logout))
                            .route(web::get().to_async(auth_handler::get_me)),
                    )
                    // routes to invitation
                    .service(
                        web::resource("/invitation")
                            .route(web::post().to_async(invitation_handler::register_email)),
                    )
                    // routes to register as a user after the
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to_async(register_handler::register_user)),
                    ),
            )
            // serve static files
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind("0.0.0.0:3000")?
    .start();

    sys.run()
}
