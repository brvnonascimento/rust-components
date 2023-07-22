use axum::{response::Html, routing::get, Router};
use html_to_string_macro::html;

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
    html!(
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
    )
}

fn heading(children: Children) -> String {
    html!(
      <h1>
          {resolve_children(children)}
      </h1>
    )
}

#[tokio::main]
async fn main() {
    // build our application with a single route

    let page = layout(Children::Function(|_children| {
        heading(Children::Function(|_| {
            "Hello world from a component!".to_string()
        }))
    }));

    let app = Router::new().route("/", get(|| async { Html(page) }));

    // run it with hyper on localhost:3100
    axum::Server::bind(&"0.0.0.0:3100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
