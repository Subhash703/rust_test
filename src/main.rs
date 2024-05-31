use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
use std::net::Ipv4Addr;
mod api;

use api::{
  user::handler::{User, get_users},
  post::handler::{Post, get_posts},
  todo::handler::{Todo, get_todos}
};

#[derive(OpenApi)]
#[openapi(
    paths(
        api::user::handler::get_users,
        api::post::handler::get_posts,
        api::todo::handler::get_todos,
    ),
    components(
        schemas(Todo, User, Post)
    ),
    tags(
        (name = "API Project", description = "This is sample project")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/apis")
                    .service(web::resource("/users").route(web::get().to(get_users)))
                    .service(web::resource("/posts").route(web::get().to(get_posts)))
                    .service(web::resource("/todos").route(web::get().to(get_todos)))
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
                Url::new("api", "/api-docs.json"),
                ApiDoc::openapi(),
            )]))
    })
    .bind((Ipv4Addr::LOCALHOST, 8989))?
    .run()
    .await
}
