use axum::{routing::get, Router};
use std::{fs::File, io::Read};
use pulldown_cmark::{Parser, Options};

#[tokio::main]
async fn main() -> std::io::Result<()>{

    /* pulldown_cmark base code */
    let mut markdown = File::open("notes/info.md")?;
    let mut contents = String::new();
    let _ = markdown.read_to_string(&mut contents);

    let mut options = Options::empty();
    let parser = Parser::new_ext(&contents, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);


    /* axum base code */
    let app = Router::new()
        .route("/", get(|| async { html_output }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
