use axum::routing::{get, post};
use axum::{extract::Json, response::Json as JsonResponse, Router};
use diesel::{prelude::*, sql_query};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
// use tower_http::cors::CorsLayer;
use tracing::{error, info};
use uuid::Uuid;

mod db;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::models::{NewTodo, Todo};
use crate::schema::todos::dsl::todos;

#[derive(Serialize, Deserialize)]
struct UserInput {
    title: String,
    description: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().compact().init();

    // Building our application with a single Route
    let app = Router::new()
        .route("/", get(get_todos))
        .route("/", post(create_todos));

    // Run the server with hyper on http://127.0.0.1:5050
    let addr = SocketAddr::from(([127, 0, 0, 1], 5050));
    info!("[!] API Server listening: http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("API Server failed to initialize!");
}

async fn get_todos() -> JsonResponse<Value> {
    info!("[!] GET Request");
    let mut connection = establish_connection();

    let data: Vec<Todo> = todos
        .load::<Todo>(&mut connection)
        .expect("Error occurred while reading database!");
    // let data: Result<Vec<Todo>> = sql_query("SELECT * FROM todos ORDER BY id").load(&mut connection);
    info!("[!] Read successful!");

    Json(json!({
        "success": "true",
        "data": data,
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

    info!("[!] Insert successful!");

    Json(json!({
        "success": "true",
        "data": new_todo,
    }))
}
