extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;

extern crate subprocess;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use hyper::rt::{self, Future, Stream};
use hyper::{Body, Client, Method, Request};
use std::env::args;
use std::io::{self, Write};
fn main() {
    pretty_env_logger::init();

    let (method, url) = match (args().nth(1), args().nth(2)) {
        (Some(method), Some(url)) => (method, url),
        _ => {
            println!("Usage: ./postwomen <get|post|put|delete|..> <url>");
            return;
        }
    };
    let url = url.parse::<hyper::Uri>().unwrap();
    rt::run(handle(method, url));
}

fn handle(method: String, url: hyper::Uri) -> impl Future<Item = (), Error = ()> {
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = Client::builder().build::<_, Body>(https);

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
    client
        .request(req)
        .and_then(|res| {
            println!("status: {}", res.status());
            res.into_body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| panic!("expect stdout is open. err: {}", e))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}
