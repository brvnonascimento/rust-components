use axum::{response::Html, routing::get, Router};

enum Children {
    String(String),
    Html(Html<String>),
    Function(fn(children: Children) -> Html<String>),
}

fn resolve_children(children: Children) -> String {
    match children {
        Children::String(s) => s,
        Children::Html(h) => h.0,
        Children::Function(f) => f(Children::String("Hello, world!".to_string())).0,
    }
}

fn layout(children: Children) -> Html<String> {
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>rust components</title>
            </head>
            <body>
                <div style="display: flex; flex-direction: column; align-items: center">
                    {}
                </div>
            </body>
        </html>
    "#,
        resolve_children(children)
    ))
}

fn heading(children: Children) -> Html<String> {
    Html(format!("<h1>{}</h1>", resolve_children(children)))
}

#[tokio::main]
async fn main() {
    // build our application with a single route

    let page = layout(Children::Function(|_children| {
        heading(Children::Function(|_| {
            Html(format!("Hello world from a component!"))
        }))
    }));

    let app = Router::new().route("/", get(|| async { page }));

    // run it with hyper on localhost:3100
    axum::Server::bind(&"0.0.0.0:3100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
