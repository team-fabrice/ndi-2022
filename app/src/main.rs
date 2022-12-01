use rocket::{fairing::{AdHoc, self}, Rocket, Build, fs::{FileServer, relative}};
use sea_orm_rocket::Database;

#[macro_use] extern crate rocket;

use migration::{self, MigratorTrait};

mod pool;
mod routes;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &pool::Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/", routes::home::routes())
}
