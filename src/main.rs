#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use dotenv::dotenv;
use env_logger;

pub mod schema;
pub mod routes;
mod errors;

use actix_web::{
    HttpServer,
    App,
    middleware::{
        Compress, 
        Logger, 
    },
    web,
    http,
};

use actix_files::Files;
use crate::routes::routes;
use crate::views::not_found;

#[macro_use]
mod utils;
#[macro_use]
mod views;
#[macro_use]
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();

        App::new()  
            .wrap(Compress::default())
            .service(_files)
            .service(_files2)
            .configure(routes)
            .default_service(web::route().to(not_found))
    })
    .bind("192.168.0.127:9010")?
    .run()
    .await
}
