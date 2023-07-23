#[macro_use]
extern crate dotenv_codegen;

use axum::{response::Html, routing::get, Extension, Router};
use dotenv::dotenv;
use html_to_string_macro::html;
use sqlx::postgres::PgPoolOptions;

enum Children {
    String(String),
    Function(fn(children: Children) -> String),
}

fn resolve_children(children: Children) -> String {
    match children {
        Children::String(s) => s,
        Children::Function(f) => f(Children::String("".to_string())),
    }
}

fn layout(children: Children) -> String {
    html! {
      <!DOCTYPE html>
      <html>
        <head>
          <title>"rust components"</title>
        </head>
        <body>
          <div style="display: flex; flex-direction: column; align-items: center">
            {resolve_children(children)}
          </div>
        </body>
      </html>
    }
}

fn heading(children: Children) -> String {
    html! {
      <h1>
        {resolve_children(children)}
      </h1>
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let page = layout(Children::Function(|_children| {
        heading(Children::Function(|_| {
            "Hello world from a component v2!".to_string()
        }))
    }));

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(dotenv!("DATABASE_URL"))
        .await;

    match db {
        Ok(db) => {
            println!("Connected to database: {}", dotenv!("POSTGRES_DB"));

            let migration = sqlx::migrate!().run(&db).await;

            match migration {
                Ok(_) => println!("Migrations ran successfully"),
                Err(e) => {
                    println!("Failed to run migrations: {}", e);
                    return;
                }
            }

            let app = Router::new()
                .route("/", get(|| async { Html(page) }))
                .layer(Extension(db));

            let address = &"0.0.0.0:3100".parse();

            match address {
                Ok(address) => {
                    let server = axum::Server::bind(address)
                        .serve(app.into_make_service())
                        .await;

                    match server {
                        Ok(_) => println!("Server started on port 3100"),
                        Err(e) => println!("Failed to start server: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to parse address: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
        }
    }
}
