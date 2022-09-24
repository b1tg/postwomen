use hyper::{Body, Client, Method, Request};
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let (method, url) = match (args().nth(1), args().nth(2)) {
        (Some(method), Some(url)) => (method, url),
        _ => {
            println!("Usage: ./postwomen <get|post|put|delete|..> <url>");
            return Ok(());
        }
    };
    let url = url.parse::<hyper::Uri>().unwrap();
    handle(method, url).await
}

async fn handle(method: String, url: hyper::Uri) -> Result<()> {
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        // .http2_only(true)
        .build_http();
    let mut req: Request<_>;
    if method.as_str() == "post" || method.as_str() == "put" {
        subprocess::Exec::cmd("vim")
            .arg("#temp.json")
            .join()
            .unwrap();

        let path = Path::new("#temp.json");
        let mut file = File::open(&path).unwrap();
        let mut json_s = String::new();
        file.read_to_string(&mut json_s).unwrap();

        req = Request::new(Body::from(json_s));
    } else {
        req = Request::new(Body::from("{}"));
    }
    // *req.method_mut() = Method::POST;
    *req.method_mut() = match method.as_str() {
        "post" => Method::POST,
        "put" => Method::PUT,
        "delete" => Method::DELETE,
        _ => Method::GET,
    };
    *req.uri_mut() = url.clone();
    req.headers_mut().insert(
        // need custom header
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static("application/json"),
    );
    let res = client.request(req).await?;
    println!("status: {}", res.status());
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    println!("body: {}", body_str);
    Ok(())
}
