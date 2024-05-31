use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub content: String,
}

#[utoipa::path(
  get,
  path = "/apis/posts",
  responses(
      (status = 200, description = "List current posts", body = [Post]),
  )
)]
pub async fn get_posts() -> impl Responder {
  let posts = vec![
      Post {
          id: 1,
          title: "Hello World".to_string(),
          content: "This is the content of the first post".to_string(),
      },
      Post {
          id: 2,
          title: "Another Post".to_string(),
          content: "This is the content of the second post".to_string(),
      },
  ];

  HttpResponse::Ok().json(posts)
}
