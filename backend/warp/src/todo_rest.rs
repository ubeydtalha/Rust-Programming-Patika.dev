use std::sync::Arc;

use serde_json::{json, Value};
use warp::{reply::Json, Filter};

use crate::{security::{do_auth, UserCtx}, with_dp_pool, DbPool};

pub fn todos_filter(db_pool : Arc<DbPool>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{

        let todos_base = warp::path("todos");

        let list = todos_base
            .and(warp::get())
            .and(warp::path::end())
            .and(do_auth())
            .and(with_dp_pool(db_pool.clone()))
            .and_then(todo_list);

        let get = todos_base
            .and(warp::get())
            .and(do_auth())
            .and(with_dp_pool(db_pool.clone()))
            .and(warp::path::param())
            .and_then(todo_get);

    async fn todo_list(
        _user_ctx : UserCtx,
        db_pool : Arc<DbPool>
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // TODO - get from db

        let todos = json!(
            [
                {"id": 1, "title": "Implement a REST API"},
                {"id": 2, "title": "?????"},
                {"id": 3, "title": "profit!"}
            ]
        );

        let todos = warp::reply::json(&todos);

        Ok(todos)
    }

    let create = todos_base
        .and(warp::post())
        .and(do_auth())
        .and(with_dp_pool(db_pool.clone()))
        .and(warp::body::json())
        .and_then(todo_create);
        

    list.or(get).or(create)
}

async fn todo_list(_user_ctx : UserCtx)
    -> Result<impl warp::Reply, warp::Rejection>
{
    // TODO - get from db

    let todos = json!(
        [
            {"id": 1, "title": "Implement a REST API"},
            {"id": 2, "title": "?????"},
            {"id": 3, "title": "profit!"}
        ]
    );

    let todos = warp::reply::json(&todos);

    Ok(todos)
}


async fn todo_get(
    _user_ctx : UserCtx,
    db_pool : Arc<DbPool>,
    id : i64
) -> Result<Json,warp::Rejection>{
    // TODO - get from db
    let todo = json!({"id": id,"user_id": _user_ctx.user_id , "title": "Implement a REST API"});
    
    let todo_json = warp::reply::json(&todo);

    Ok(todo_json)
}

async fn todo_create(
    _user_ctx : UserCtx,
    db_pool : Arc<DbPool>,
    data : Value
) -> Result<Json,warp::Rejection>{

    // TODO - create in db
    let todo = data;

    let todo_json = warp::reply::json(&todo);

    Ok(todo_json)

}