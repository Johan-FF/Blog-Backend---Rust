#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("db_url not found");

    let mut conn: PgConnection = PgConnection::establish(&db_url).expect("db connection failed");

    use self::models::{NewPost, Post, SpecificPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost {
        title: "Mi primer blog",
        body: "Lorem ipsum",
        slug: "primer-post",
    };
    let _post: Post = diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(&mut conn)
        .expect("insert failed");

    // Select * from posts
    let mut post_result = posts.load::<Post>(&mut conn).expect("query error");
    for post in post_result {
        println!("{:?}", post);
    }

    post_result = posts.limit(1).load::<Post>(&mut conn).expect("query error");
    for post in post_result {
        println!("{:?}", post);
    }

    let specific_post_result = posts
        .select((title, body))
        .load::<SpecificPost>(&mut conn)
        .expect("query error");
    for post in specific_post_result {
        println!("{:?}", post);
    }

    post_result = posts
        .order(id.desc())
        .limit(1)
        .load::<Post>(&mut conn)
        .expect("query error");
    for post in post_result {
        println!("{:?}", post);
    }

    let mut post_result = posts
        .filter(id.eq(4))
        .limit(1)
        .load::<Post>(&mut conn)
        .expect("query error");
    for post in post_result {
        println!("{:?}", post);
    }

    let mut _post_update = diesel::update(posts.filter(id.eq(4)))
        .set((body.eq("Lorem ipsum"), slug.eq("primer-post")))
        .get_result::<Post>(&mut conn)
        .expect("update error");
    _post_update = diesel::update(posts.filter(id.eq(4)))
        .set(slug.eq("primer-post"))
        .get_result::<Post>(&mut conn)
        .expect("update error");

    post_result = posts
        .filter(id.eq(4))
        .limit(1)
        .load::<Post>(&mut conn)
        .expect("query error");
    for post in post_result {
        println!("{:?}", post);
    }

    diesel::delete(posts.filter(id.eq(4)))
        .execute(&mut conn)
        .expect("delete error");

    diesel::delete(posts.filter(title.like("%-blog%")))
        .execute(&mut conn)
        .expect("delete error");
}
