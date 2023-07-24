use dotenv::dotenv;
use std::path::Path;

fn main() {
    dotenv().ok();

    hyperide::tailwind::bootstrap(
        Path::new("./tailwind.config.js"),
        Path::new("./src/styles/tailwind.css"),
    );
}
