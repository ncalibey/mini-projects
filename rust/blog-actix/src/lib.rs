#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod models;
mod routes;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16) -> Self {
        Blog { port }
    }

    pub fn run(&self, database_url: String) -> std::io::Result<()> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        // `Pool` is just an `Arc` wrapper around the struct that manages connections.
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                // `configure` takes an argument that satisfies the `FnOnce(&mut ServiceConfig)`
                // trait bound, meaning we guarantee it's okay to only call it once. We can thus
                // define our routes in another module and rig them up here.
                .configure(routes::users::configure)
                .configure(routes::posts::configure)
                .configure(routes::comments::configure)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
    }
}
