use std::io::Write;

pub use backend::routes::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let doc = ApiDoc::openapi().to_pretty_json().unwrap();

    let file_name = match std::env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            eprintln!("Usage: cargo run --bin gen_schema -- <file_name>");
            std::process::exit(1);
        }
    };
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(doc.as_bytes()).unwrap();
}
