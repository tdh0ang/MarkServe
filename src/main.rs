use axum::{Router, response::Html, routing::get};
use std::{fs::File, io::{Read, Write}};
use pulldown_cmark::{Parser, Options};
use walkdir::WalkDir;


fn print_dir() -> std::io::Result<()>{
    for entry in WalkDir::new("notes/") {
        println!("{}", entry?.path().display());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()>{

    let _ = print_dir();

    /* pulldown_cmark base code */
    let mut markdown = File::open("notes/info.md")?;
    let mut contents = String::new();
    let _ = markdown.read_to_string(&mut contents);

    let options = Options::empty();
    let parser = Parser::new_ext(&contents, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    /* axum base code */
    let app = Router::new().route("/", get(Html(html_output)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[INFO] Server starting...");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}


