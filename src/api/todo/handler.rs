use std::vec;

use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Todo {
    pub title: String,
    pub is_completed: bool,
    pub description: String,
}

#[utoipa::path(
  get,
  path="/apis/todos",
  responses (
    (status = 200, description = "List current posts", body = [Todo]),
  )
)]
pub async fn get_todos() -> impl Responder {
  let todos = vec![
      Todo {
        title: "Read a book".to_string(),
        is_completed: false,
        description: "It's a good book".to_string(),
      },
      Todo {
        title: "Go to gym".to_string(),
        is_completed: false,
        description: "Why not?".to_string(),
      }
      ];

  HttpResponse::Ok().json(todos)
}