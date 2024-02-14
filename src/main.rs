//#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use self::models::{NewPostHandler, Post};
//use self::schema::posts;
use self::schema::posts::dsl::*;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::{ConnectionManager, Pool};

use crate::web::{block, Data, Json};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("db connection error in index");
    match block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(error) => HttpResponse::Ok().body(format!("{:?}", error)),
    }
}

#[post("/new-post")]
async fn new_post(pool: Data<DbPool>, item: Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("db connection error in index");

    match block(move || Post::create_post(&mut conn, &item)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(error) => HttpResponse::Ok().body(format!("{:?}", error)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("db_url not found");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("pool error");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(new_post)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("localhost", 9900))
    .unwrap()
    .run()
    .await
}
