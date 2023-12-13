use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_hello_paradigma_world(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("company", "Paradigma")
        .body(Some("Hello, Paradigma World!\n".into()))?)
}
