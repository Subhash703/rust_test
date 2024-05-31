use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub email: String,
}

#[utoipa::path(
  get,
  path = "/apis/users",
  responses(
      (status = 200, description = "List all users", body = [User]),
  )
)]
pub async fn get_users() -> impl Responder {
  let users = vec![
      User {
          id: 1,
          name: "Subhash Chandra".to_string(),
          email: "Subhash.chandra@juspay.in".to_string(),
      }
  ];

  HttpResponse::Ok().json(users)
}