mod todo_rest;
mod security;
use std::sync::Arc;

use warp::Filter;

use crate::todo_rest::todos_filter;

const HELLO_WORLD: &str = "web-folder/";

#[tokio::main]
async fn main() {
    let db_pool = Arc::new(DbPool{});

    //APIs
    let hi = warp::path("hi").and(warp::get()).map(
        || "Hi!"
    );
    let apis = hi.or(todos_filter(db_pool.clone()));

    //  Static Content
    let content = warp::fs::dir(HELLO_WORLD);
    let root = warp::get().and(warp::path::end())
    .and(warp::fs::file(format!("{}index.html", HELLO_WORLD)));
    let static_site = content.or(root);
    
    let routes = apis.or(static_site);

    println!("Start web-server");
    warp::serve(
        routes
    ).run(([127, 0, 0, 1], 3030)).await;
}


pub struct DbPool{

}

pub fn with_dp_pool(
    dp_pool: Arc<DbPool>
) -> impl Filter<Extract = (Arc<DbPool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || dp_pool.clone())
}