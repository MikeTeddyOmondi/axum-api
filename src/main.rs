// use axum::http::Request;
// use axum::middleware::{self, Next};
// use axum::response::Response;
// use axum::extract::Query;
use axum::routing::{delete, get, post, put};
use axum::{
    extract::{Json, Path},
    response::Json as JsonResponse,
    Router,
};
// use diesel::sql_query;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use uuid::Uuid;

mod db;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::models::{NewTodo, Todo};
use crate::schema::todos::completed;
use crate::schema::todos::dsl::todos;

#[derive(Serialize, Deserialize)]
struct UserInput {
    title: String,
    description: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().compact().init();

    // Custom middleware
    // async fn cors_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    //     let cors = CorsLayer::permissive();
    //     let response = next(request);
    // }

    let cors = CorsLayer::permissive();

    // Building our application with a single Route
    let app = Router::new()
        .route("/", get(get_api_info))
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todos))
        .route("/todos/:public_id", get(get_todo))
        .route("/todos/:public_id", put(complete_todo))
        .route("/todos/:public_id", delete(delete_todo))
        .route("/todos/clear", delete(delete_all_todos))
        .layer(cors); // tower-http 0.5 Not yet supported for

    // Run the server with hyper on http://127.0.0.1:5050
    let addr = SocketAddr::from(([127, 0, 0, 1], 5050));
    info!("[!] API Server listening: http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("API Server failed to initialize!");
}

async fn get_api_info() -> JsonResponse<Value> {
    info!("[!] GET Request: API Info");

    Json(json!({
        "apiVersion": "v1.0",
        "description": "Todos RESTful API built on Axum",
    }))
}

async fn get_todos() -> JsonResponse<Value> {
    info!("[!] GET Request");
    let mut connection = establish_connection();

    let data: Vec<Todo> = todos
        .load::<Todo>(&mut connection)
        .expect("Error occurred while reading database!");
    // let data: Result<Vec<Todo>> = sql_query("SELECT * FROM todos ORDER BY id").load(&mut connection);

    let _serialized_data = to_string(&data).unwrap();

    info!("[!] Read all successful!");
    info!("[!] Data: {:?} todos", data.len());

    Json(json!({
        "success": "true",
        "data": data,
    }))
}

async fn get_todo(Path(public_id): Path<Uuid>) -> JsonResponse<Value> {
    info!("[!] GET Request: {}", public_id);
    let mut connection = establish_connection();

    let data: Vec<Todo> = todos
        .filter(schema::todos::public_id.eq(public_id.to_string()))
        .load::<Todo>(&mut connection)
        .expect("Error occurred while reading database!");

    if data.len() == 0 {
        let err = Json(json!({
            "success": "false",
            "data": { "id": public_id.to_string(), "message": "Todo not found!" }
        }));
        error!("[#] Error: {public_id} not found!");
        return err;
    }

    let serialized_data = to_string(&data).unwrap();

    info!("[!] Read: {} successful!", public_id);
    info!("[!] Data: {:?}", serialized_data);

    Json(json!({
        "success": "true",
        "params": {
            "id": public_id.to_string()
        },
        "data": data,
    }))
}

async fn complete_todo(Path(public_id): Path<Uuid>) -> JsonResponse<Value> {
    info!("[!] PUT Request: {}", public_id);
    let mut connection = establish_connection();

    let data: Vec<Todo> = todos
        .filter(schema::todos::public_id.eq(public_id.to_string()))
        .load::<Todo>(&mut connection)
        .expect("Error occurred while reading database!");

    if data.len() == 0 {
        let err = Json(json!({
            "success": "false",
            "data": { "id": public_id.to_string(), "message": "Todo not found!" }
        }));
        error!("[#] Error: {public_id} not found!");
        return err;
    }

    let todo = &data[0];

    if todo.completed == 0 {
        let updated_rows = diesel::update(todos)
            .filter(schema::todos::public_id.eq(public_id.to_string()))
            .set(completed.eq(1))
            .execute(&mut connection)
            .unwrap();

        info!("[!] Update: {} successful!", public_id);
        info!("[!] Data: {:?} row completed", updated_rows);

        return Json(json!({
            "success": "true",
            "params": {
                "id": public_id.to_string()
            },
            "data": { "message": "Todo completed!" },
        }));
    } else if todo.completed == 1 {
        let updated_rows = diesel::update(todos)
            .filter(schema::todos::public_id.eq(public_id.to_string()))
            .set(completed.eq(0))
            .execute(&mut connection)
            .unwrap();

        info!("[!] Update: {} successful!", public_id);
        info!("[!] Data: {:?} row restored", updated_rows);

        return Json(json!({
            "success": "true",
            "params": {
                "id": public_id.to_string()
            },
            "data": { "message": "Todo restored!" },
        }));
    } else {
        info!("[!] Update: {} unsuccessful!", public_id);

        return Json(json!({
            "success": "false",
            "params": {
                "id": public_id.to_string()
            },
            "data": { "message": "Error occurred while updating todo!" },
        }));
    }
}

async fn delete_todo(Path(public_id): Path<Uuid>) -> JsonResponse<Value> {
    info!("[!] DELETE Request: {}", public_id);
    let mut connection = establish_connection();

    let data: Vec<Todo> = todos
        .filter(schema::todos::public_id.eq(public_id.to_string()))
        .load::<Todo>(&mut connection)
        .expect("Error occurred while reading database!");

    if data.len() == 0 {
        let err = Json(json!({
            "success": "false",
            "data": { "id": public_id.to_string(), "message": "Todo not found!" }
        }));
        error!("[#] Error: {public_id} not found!");
        return err;
    }

    let deleted_rows: usize =
        diesel::delete(todos.filter(schema::todos::public_id.eq(&public_id.to_string())))
            .execute(&mut connection)
            .unwrap();

    info!("[!] Delete: {} successful!", public_id);
    info!("[!] Data: {:?} row deleted", deleted_rows);

    Json(json!({
        "success": "true",
        "params": {
            "id": public_id.to_string()
        },
        "data": { "message": "Todo deleted!" },
    }))
}

async fn delete_all_todos() -> JsonResponse<Value> {
    info!("[!] DELETE Request");
    let mut connection = establish_connection();

    let deleted_rows: usize = diesel::delete(todos).execute(&mut connection).unwrap();

    info!("[!] Delete all successful!");
    info!("[!] Data: {:?} row deleted", deleted_rows);

    Json(json!({
        "success": "true",
        "data": { "message": "Todos cleared!" },
    }))
}

async fn create_todos(Json(req_body): Json<UserInput>) -> JsonResponse<Value> {
    info!("[!] GET Request");
    let rb: UserInput = req_body;

    let mut connection = establish_connection();

    if rb.title.is_empty() || rb.description.is_empty() {
        let err = Json(json!({
            "success": "false",
            "data": { "message": "Enter all fields" }
        }));
        error!("[#] Error: enter all fields!");
        return err;
    };

    let p_id = Uuid::new_v4().to_string();
    let new_todo = NewTodo {
        public_id: &p_id,
        title: &rb.title,
        description: &rb.description,
    };

    diesel::insert_into(todos)
        .values(&new_todo)
        .execute(&mut connection)
        .expect("Error occurred while inserting todo");

    info!("[!] Create: {} successful!", new_todo.public_id);

    Json(json!({
        "success": "true",
        "data": new_todo,
    }))
}
