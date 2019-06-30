// main.rs
#![allow(dead_code)] // usful in dev mode
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
use actix::prelude::*;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;

mod errors;
mod models;
mod schema;

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
        App::new()
            // add database pool as data/state to the app
            .data(address.clone())
            // setup logger for requests
            .wrap(Logger::default())
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    // routes for authentication
                    .service(web::resource("/auth").route(web::get().to(|| {})))
                    // routes to invitation
                    .service(web::resource("/invitation").route(web::get().to(|| {})))
                    // routes to register as a user after the
                    .service(
                        web::resource("/register/{invitation_id}").route(web::get().to(|| {})),
                    ),
            )
    })
    .bind("127.0.0.1:3000")?
    .start();

    sys.run()
}
