#[macro_use]
extern crate dotenv_codegen;

use axum::{response::Html, routing::get, Extension, Router};
use hyperide::{htmx::include_htmx, hyperide, tailwind::include_tailwind};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

enum Children {
    String(String),
    Function(fn(children: Children) -> String),
    Vec(Vec<Children>),
}

fn resolve_children(children: Children) -> String {
    match children {
        Children::String(s) => s,
        Children::Function(f) => f(Children::String("".to_string())),
        Children::Vec(v) => {
            let mut result = String::new();

            for child in v {
                result.push_str(&resolve_children(child));
            }

            result
        }
    }
}

fn layout(children: Children) -> String {
    hyperide! {
      <!DOCTYPE html>
      <html>
        <head>
          <title>rust components</title>

          { include_tailwind!() }
          { include_htmx!() }
        </head>
        <body>
          <div
            class="container mx-auto px-4 py-4 bg-gray-100 rounded-lg shadow-lg"
          >
            {resolve_children(children)}
          </div>
        </body>
      </html>
    }
}

fn heading(children: Children) -> String {
    hyperide! {
      <h1>
        {resolve_children(children)}
      </h1>
    }
}

#[tokio::main]
async fn main() {
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(dotenv!("DATABASE_URL"))
        .await;

    match db {
        Ok(db) => {
            println!("Connected to database!");

            let migration = sqlx::migrate!().run(&db).await;

            match migration {
                Ok(_) => println!("Migrations ran successfully"),
                Err(e) => {
                    println!("Failed to run migrations: {}", e);
                    return;
                }
            }

            let users = sqlx::query!(
                r#"
                    SELECT * from "user"
                "#
            )
            .fetch_all(&db)
            .await
            .unwrap();

            let mut user = String::new();

            for u in users {
                user.push_str(&format!("{}<br>", u.name));
            }

            let page = layout(Children::Vec(vec![
                Children::Function(|_children| {
                    heading(Children::Function(|_| "Users".to_string()))
                }),
                Children::String(user),
            ]));

            let app = Router::new()
                .route("/", get(|| async { Html(page) }))
                .layer(Extension(db));

            let address = SocketAddr::from(([127, 0, 0, 1], 3000));

            let server = axum::Server::bind(&address)
                .serve(app.into_make_service())
                .await;

            match server {
                Ok(_) => println!("Server started on port 3100"),
                Err(e) => println!("Failed to start server: {}", e),
            }
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
        }
    }
}
